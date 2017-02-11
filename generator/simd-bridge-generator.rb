#!/usr/bin/env ruby

$: << "#{__dir__}/lib"

require "bridge/output"

module Bridge
  class SIMD
    TYPES = [
      { size: 1, max_width: 4, type: 'i8', opencl: 'char', kind: %i(signed integer), bool: "char" },
      { size: 1, max_width: 4, type: 'u8', opencl: 'uchar', kind: %i(unsigned integer), bool: "char" },
      { size: 2, max_width: 4, type: 'i16', opencl: "short", kind: %i(signed integer), bool: "short" },
      { size: 2, max_width: 4, type: 'u16', opencl: "ushort", kind: %i(unsigned integer), bool: "short" },
      { size: 4, max_width: 4, type: 'i32', opencl: "int", kind: %i(signed integer), bool: "int" },
      { size: 4, max_width: 4, type: 'u32', opencl: "uint", kind: %i(unsigned integer), bool: "int" },
      { size: 4, max_width: 4, type: 'f32', opencl: "float", kind: %i(float), bool: "int", max_matrix_size: 4 },
      { size: 8, max_width: 4, type: 'i64', opencl: "long", kind: %i(signed integer), bool: "long" },
      { size: 8, max_width: 4, type: 'u64', opencl: "ulong", kind: %i(unsigned integer), bool: "long"  },
      { size: 8, max_width: 4, type: 'f64', opencl: "double", kind: %i(float), bool: "long", max_matrix_size: 4 }
    ]

    TYPES_BY_NAME = TYPES.map { |x| [x[:opencl], x] }.to_h

    WIDTHS = [1, 2, 3, 4, 8, 16]

    def self.generate(path)
      FileUtils.mkdir_p(path)

      files = []

      TYPES.each do |attributes|
        scalar = attributes.fetch(:type)
        type = attributes.fetch(:opencl)
        kind = attributes.fetch(:kind)
        bool = attributes.fetch(:bool)

        WIDTHS.select { |x| x <= attributes.fetch(:max_width) }.each do |width|
          io = StringIO.new
          o = Bridge::Output.new(io)

          name = "#{type}#{width}"
          bool_name = "#{bool}#{width}"

          if width == 1
            o.puts("pub type #{name} = #{attributes.fetch(:type)};", pad: true)
          else
            content = (["pub #{attributes.fetch(:type)}"] * width).join(", ")

            o.puts("use std;", pad: true)
            o.puts("use ::*;")

            o.puts("#[repr(C)]", pad: true)
            o.puts("#[repr(simd)]")
            o.puts("#[derive(Copy, Clone, Debug)]")
            o.puts("pub struct #{name}(#{content});")

            o.block("extern \"platform-intrinsic\"", pad: true) do
              o.puts("fn simd_add<T>(x: T, y: T) -> T;", pad: true)
              o.puts("fn simd_sub<T>(x: T, y: T) -> T;")
              o.puts("fn simd_mul<T>(x: T, y: T) -> T;")
              o.puts("fn simd_div<T>(x: T, y: T) -> T;")

              if kind.include?(:integer)
                o.puts("fn simd_shl<T>(x: T, y: T) -> T;", pad: true)
                o.puts("fn simd_shr<T>(x: T, y: T) -> T;")

                o.puts("fn simd_and<T>(x: T, y: T) -> T;", pad: true)
                o.puts("fn simd_or<T>(x: T, y: T) -> T;")
                o.puts("fn simd_xor<T>(x: T, y: T) -> T;")
              end

              o.puts("fn simd_eq<T, U>(x: T, y: T) -> U;", pad: true)
              o.puts("fn simd_ne<T, U>(x: T, y: T) -> U;")
              o.puts("fn simd_lt<T, U>(x: T, y: T) -> U;")
              o.puts("fn simd_le<T, U>(x: T, y: T) -> U;")
              o.puts("fn simd_gt<T, U>(x: T, y: T) -> U;")
              o.puts("fn simd_ge<T, U>(x: T, y: T) -> U;")

              o.puts("fn simd_cast<T, U>(x: T) -> U;", pad: true)

              o.puts("fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;", pad: true)
              o.puts("fn simd_extract<T, E>(x: T, i: u32) -> E;")
            end

            o.block("impl std::ops::Index<u32> for #{name}", pad: true) do |o|
              o.puts("type Output = #{scalar};")
              o.puts
              o.puts("#[inline]")
              o.block("fn index(&self, index: u32) -> &#{scalar}") do |o|
                o.puts("return unsafe { simd_extract(self, index) };")
              end
            end

            %w(add sub mul div).each do |op|
              o.block("impl std::ops::#{op.capitalize} for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: Self) -> Self") do |o|
                  o.puts("return unsafe { simd_#{op}(self, other) };")
                end
              end

              o.block("impl std::ops::#{op.capitalize}<#{scalar}> for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: #{scalar}) -> Self") do |o|
                  o.puts("return unsafe { simd_#{op}(self, #{name}::broadcast(other)) };")
                end
              end

              o.block("impl std::ops::#{op.capitalize}<#{name}> for #{scalar}", pad: true) do |o|
                o.puts("type Output = #{name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: #{name}) -> #{name}") do |o|
                  o.puts("return unsafe { simd_#{op}(#{name}::broadcast(self), other) };")
                end
              end
            end

            if kind.include?(:integer)
              %w(and or xor).each do |op|
                o.block("impl std::ops::Bit#{op.capitalize} for #{name}", pad: true) do |o|
                  o.puts("type Output = Self;")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn bit#{op}(self, other: Self) -> Self") do |o|
                    o.puts("return unsafe { simd_#{op}(self, other) };")
                  end
                end

                o.block("impl std::ops::Bit#{op.capitalize}<#{scalar}> for #{name}", pad: true) do |o|
                  o.puts("type Output = Self;")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn bit#{op}(self, other: #{scalar}) -> Self") do |o|
                    o.puts("return unsafe { simd_#{op}(self, #{name}::broadcast(other)) };")
                  end
                end

                o.block("impl std::ops::Bit#{op.capitalize}<#{name}> for #{scalar}", pad: true) do |o|
                  o.puts("type Output = #{name};")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn bit#{op}(self, other: #{name}) -> #{name}") do |o|
                    o.puts("return unsafe { simd_#{op}(#{name}::broadcast(self), other) };")
                  end
                end
              end

              %w(shl shr).each do |op|
                o.block("impl std::ops::#{op.capitalize}<#{name}> for #{name}", pad: true) do |o|
                  o.puts("type Output = Self;")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn #{op}(self, other: Self) -> Self") do |o|
                    o.puts("return unsafe { simd_#{op}(self, other) };")
                  end
                end

                o.block("impl std::ops::#{op.capitalize}<#{scalar}> for #{name}", pad: true) do |o|
                  o.puts("type Output = Self;")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn #{op}(self, other: #{scalar}) -> Self") do |o|
                    o.puts("return unsafe { simd_#{op}(self, #{name}::broadcast(other)) };")
                  end
                end

                o.block("impl std::ops::#{op.capitalize}<#{name}> for #{scalar}", pad: true) do |o|
                  o.puts("type Output = #{name};")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn #{op}(self, other: #{name}) -> #{name}") do |o|
                    o.puts("return unsafe { simd_#{op}(#{name}::broadcast(self), other) };")
                  end
                end
              end

              o.block("impl std::ops::Not for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn not(self) -> Self") do |o|
                  if kind.include?(:unsigned)
                    o.puts("return self ^ std::#{scalar}::MAX;")
                  else
                    o.puts("return self ^ -1;")
                  end
                end
              end
            end

            o.block("impl PartialEq for #{name}", pad: true) do |o|
              o.puts("#[inline]", pad: true)
              o.block("fn eq(&self, other: &Self) -> bool") do |o|
                o.puts("return simd::all(simd::eq(*self, *other));")
              end

              o.puts("#[inline]", pad: true)
              o.block("fn ne(&self, other: &Self) -> bool") do |o|
                o.puts("return simd::all(simd::ne(*self, *other));")
              end
            end

            o.block("impl simd::Vector for #{name}", pad: true) do |o|
              o.puts("type Scalar = #{scalar};", pad: true)
              o.puts("type Boolean = #{bool_name};")

              o.puts("#[inline(always)]", pad: true)
              o.block("fn extract(self, i: u32) -> Self::Scalar") do |o|
                o.puts("return unsafe { simd_extract(self, i) };")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn replace(self, i: u32, x: Self::Scalar) -> Self") do |o|
                o.puts("return unsafe { simd_insert(self, i, x) };")
              end

              %w(eq ne lt le gt ge).each do |op|
                o.puts("#[inline(always)]", pad: true)
                o.block("fn #{op}(self, other: Self) -> Self::Boolean") do |o|
                  o.puts("return unsafe { simd_#{op}(self, other) };")
                end
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn abs(self) -> Self") do |o|
                if kind.include?(:signed)
                  o.puts("let mask = self >> #{attributes.fetch(:size) * 8 - 1};")

                  o.puts("return (self ^ mask) - mask;")
                elsif kind.include?(:float)
                  o.puts("return simd::bitselect(#{bool_name}::broadcast(std::#{TYPES_BY_NAME[bool][:type]}::MAX), #{name}::broadcast(0.0), self);")
                else
                  o.puts("return self;")
                end
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn max(self, other: Self) -> Self") do |o|
                if kind.include?(:float)
                  result = width.times.map { |i| "self.#{i}.max(other.#{i})" }.join(", ")

                  o.puts("return #{name}(#{result});")
                else
                  o.puts("return simd::bitselect(simd::gt(other, self), self, other);")
                end
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn min(self, other: Self) -> Self") do |o|
                if kind.include?(:float)
                  result = width.times.map { |i| "self.#{i}.min(other.#{i})" }.join(", ")

                  o.puts("return #{name}(#{result});")
                else
                  o.puts("return simd::bitselect(simd::lt(other, self), self, other);")
                end
              end
            end

            o.block("impl simd::Dot for #{name}", pad: true) do |o|
              o.puts("type Output = #{scalar};")
              o.puts
              o.puts("#[inline(always)]")
              o.block("fn dot(self, other: Self) -> Self::Output") do |o|
                o.puts("return simd::reduce_add(self * other);")
              end
            end
            
            if kind.include?(:float)
              o.block("impl simd::Float for #{name}", pad: true) do |o|
                o.puts("#[inline(always)]", pad: true)
                o.block("fn copysign(self, magnitude: Self) -> Self") do |o|
                  o.puts("return simd::bitselect(#{bool_name}::broadcast(std::#{TYPES_BY_NAME[bool][:type]}::MAX), magnitude, self);")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn sign(self) -> Self") do |o|
                  o.puts("let (zero, one) = (#{name}::broadcast(0.0), #{name}::broadcast(1.0));")
                  o.puts
                  o.puts("return simd::bitselect(simd::eq(self, zero) | simd::ne(self, self), one.copysign(self), zero);")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn sqrt(self) -> Self") do |o|
                  result = width.times.map { |i| "self.#{i}.sqrt()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn recip(self) -> Self") do |o|
                  o.puts("return 1.0 / self;")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn rsqrt(self) -> Self") do |o|
                  o.puts("return self.sqrt().recip();")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn fract(self) -> Self") do |o|
                  result = width.times.map { |i| "self.#{i}.fract()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn ceil(self) -> Self") do |o|
                  result = width.times.map { |i| "self.#{i}.ceil()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn floor(self) -> Self") do |o|
                  result = width.times.map { |i| "self.#{i}.floor()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn trunc(self) -> Self") do |o|
                  result = width.times.map { |i| "self.#{i}.trunc()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn mix(self, a: Self, b: Self) -> Self") do |o|
                  o.puts("return a + self * (b - a);")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn step(self, edge: Self) -> Self") do |o|
                  o.puts("return simd::bitselect(simd::lt(self, edge), #{name}::broadcast(1.0), #{name}::broadcast(0.0));")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn smoothstep(self, edge0: Self, edge1: Self) -> Self") do |o|
                  o.puts("let t = simd::clamp((self - edge0) / (edge1 - edge0), #{name}::broadcast(0.0), #{name}::broadcast(1.0));")
                  o.puts
                  o.puts("return t * t * (3.0 - 2.0 * t);")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn sin(self) -> Self") do |o|
                  result = width.times.map { |i| "self.#{i}.sin()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn cos(self) -> Self") do |o|
                  result = width.times.map { |i| "self.#{i}.cos()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end
              end
            end

            if kind.include?(:integer)
              o.block("impl simd::Logic for #{name}", pad: true) do |o|
                constant = kind.include?(:signed) ? "std::#{scalar}::MIN" : "0x8#{"0" * (attributes.fetch(:size) * 2 - 1)}"

                o.puts("#[inline(always)]", pad: true)
                o.block("fn all(self) -> bool") do |o|
                  result = width.times.map { |i| "self.#{i}" }.join(" & ")

                  o.puts("return (#{result}) & #{constant} != 0;")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn any(self) -> bool") do |o|
                  result = width.times.map { |i| "self.#{i}" }.join(" | ")

                  o.puts("return (#{result}) & #{constant} != 0;")
                end
              end
            end

            o.block("impl simd::Reduce for #{name}", pad: true) do |o|
              o.puts("#[inline(always)]", pad: true)
              o.block("fn reduce_add(self) -> Self::Scalar") do |o|
                case width
                when 2
                  o.puts("return self.0 + self.1;")
                when 3
                  o.puts("return self.0 + self.1 + self.2;")
                else
                  o.puts("return simd::reduce_add(self.lo() + self.hi());")
                end
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn reduce_min(self) -> Self::Scalar") do |o|
                case width
                when 2
                  if kind.include?(:float)
                    o.puts("return self.0.min(self.1);")
                  else
                    o.puts("return std::cmp::min(self.0, self.1);")
                  end
                when 3
                  if kind.include?(:float)
                    o.puts("return self.2.min(simd::reduce_min(self.lo()));")
                  else
                    o.puts("return std::cmp::min(simd::reduce_min(self.lo()), self.2);")
                  end
                else
                  o.puts("return simd::reduce_min(simd::min(self.lo(), self.hi()));")
                end
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn reduce_max(self) -> Self::Scalar") do |o|
                case width
                when 2
                  if kind.include?(:float)
                    o.puts("return self.0.max(self.1);")
                  else
                    o.puts("return std::cmp::max(self.0, self.1);")
                  end
                when 3
                  if kind.include?(:float)
                    o.puts("return self.2.max(simd::reduce_max(self.lo()));")
                  else
                    o.puts("return std::cmp::max(simd::reduce_max(self.lo()), self.2);")
                  end
                else
                  o.puts("return simd::reduce_max(simd::max(self.lo(), self.hi()));")
                end
              end
            end

            if kind.include?(:signed)
              TYPES.select { |t| t.fetch(:bool) == type }.each do |other|
                other_name = "#{other.fetch(:opencl)}#{width}"

                o.block("impl simd::Select<#{other_name}> for #{name}", pad: true) do |o|
                  o.puts("#[inline(always)]", pad: true)
                  o.block("fn select(self, a: #{other_name}, b: #{other_name}) -> #{other_name}") do |o|
                    o.puts("return (self >> #{attributes.fetch(:size) * 8 - 1}).bitselect(a, b);")
                  end

                  o.puts("#[inline(always)]", pad: true)
                  o.block("fn bitselect(self, a: #{other_name}, b: #{other_name}) -> #{other_name}") do |o|
                    if name == other_name
                      o.puts("return (a & !self) | (b & self);")
                    else
                      o.puts("return #{other_name}::bitcast(self.bitselect(#{bool_name}::bitcast(a), #{bool_name}::bitcast(b)));")
                    end
                  end
                end
              end
            end

            o.block("impl #{name}", pad: true) do |o|
              o.puts("#[inline]", pad: true)
              o.block("pub fn bitcast<T>(x: T) -> #{name}") do |o|
                o.puts("assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());")
                o.puts
                o.puts("return unsafe { std::mem::transmute_copy(&x) };")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn broadcast(x: #{scalar}) -> Self") do |o|
                o.puts("return #{name}(#{(["x"] * width).join(", ")});")
              end

              # Additions

              o.puts("#[inline]", pad: true)
              o.block("pub fn madd(x: #{name}, y: #{name}, z: #{name}) -> #{name}") do |o|
                o.puts("return x * y + z;")
              end

              # Geometry

              if kind.include?(:float)
                o.puts("#[inline]", pad: true)
                o.block("pub fn dot(x: #{name}, y: #{name}) -> #{scalar}") do |o|
                  o.puts("return simd::reduce_add(x * y);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn project(x: #{name}, y: #{name}) -> #{name}") do |o|
                  o.puts("return simd::dot(x, y) / simd::dot(y, y) * y;")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn length(x: #{name}) -> #{scalar}") do |o|
                  o.puts("return #{name}::length_squared(x).sqrt();")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn length_squared(x: #{name}) -> #{scalar}") do |o|
                  o.puts("return #{name}::dot(x, x);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn norm_one(x: #{name}) -> #{scalar}") do |o|
                  o.puts("return simd::reduce_add(simd::abs(x));")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn norm_inf(x: #{name}) -> #{scalar}") do |o|
                  o.puts("return simd::reduce_max(simd::abs(x));")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn distance(x: #{name}, y: #{name}) -> #{scalar}") do |o|
                  o.puts("return #{name}::length(x - y);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn distance_squared(x: #{name}, y: #{name}) -> #{scalar}") do |o|
                  o.puts("return #{name}::length_squared(x - y);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn normalize(x: #{name}) -> #{name}") do |o|
                  o.puts("return x * simd::rsqrt(#{name}::broadcast(#{name}::length_squared(x)));")
                end

                case width
                when 2
                  o.puts("#[inline]", pad: true)
                  o.block("pub fn cross(x: #{name}, y: #{name}) -> #{type}3") do |o|
                    o.puts("return #{type}3(0.0, 0.0, x.0 * y.1 - x.1 * y.0);")
                  end
                when 3
                  o.puts("#[inline]", pad: true)
                  o.block("pub fn cross(x: #{name}, y: #{name}) -> #{name}") do |o|
                    o.puts("let a = x * #{name}(y.2, y.1, y.0) - #{name}(x.2, x.1, x.0) * y;")

                    o.puts("return #{name}(a.2, a.1, a.0);")
                  end
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn reflect(x: #{name}, n: #{name}) -> #{name}") do |o|
                  o.puts("return x - 2.0 * #{name}::dot(x, n) * n;")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn refract(x: #{name}, n: #{name}, eta: #{scalar}) -> #{name}") do |o|
                  o.puts("let dp = #{name}::dot(x, n);")
                  o.puts("let k = 1.0 - eta * eta * (1.0 - dp * dp);")

                  o.puts("return if k >= 0.0 { eta * x - (eta * dp + k.sqrt()) } else { #{name}::broadcast(0.0) };")
                end

                # TODO: Link these extern functions / implement in Rust
                # o.puts("#[inline]", pad: true)
                # o.block("pub fn orient(x: #{name}, y: #{name}) -> #{name}") do |o|
                #   o.puts("let a = x * float3(y.2, y.1, y.0) - float3(x.2, x.1, x.0) * y;")
                #
                #   o.puts("return #{name}(a.2, a.1, a.0);")
                # end
                #
                # case width
                # when 2
                #   o.puts("#[inline]", pad: true)
                #   o.block("pub fn incircle(x: #{name}, y: #{name}) -> #{name}") do |o|
                #     o.puts("return #{name}(0.0, 0.0, x.0 * y.1 - x.1 * y.0);")
                #   end
                # when 3
                #   o.puts("#[inline]", pad: true)
                #   o.block("pub fn insphere(x: #{name}, y: #{name}) -> #{name}") do |o|
                #     o.puts("let a = x * float3(y.2, y.1, y.0) - float3(x.2, x.1, x.0) * y;")
                #
                #     o.puts("return #{name}(a.2, a.1, a.0);")
                #   end
                # end
              end

              # Conversion
              
              self.conversion(o, type, "char", width)
              self.conversion(o, type, "char", width, saturate: true)
              self.conversion(o, type, "uchar", width)
              self.conversion(o, type, "uchar", width, saturate: true)
              self.conversion(o, type, "short", width)
              self.conversion(o, type, "short", width, saturate: true)
              self.conversion(o, type, "ushort", width)
              self.conversion(o, type, "ushort", width, saturate: true)
              self.conversion(o, type, "int", width)
              self.conversion(o, type, "int", width, saturate: true)
              self.conversion(o, type, "uint", width)
              self.conversion(o, type, "uint", width, saturate: true)
              self.conversion(o, type, "float", width)
              self.conversion(o, type, "long", width)
              self.conversion(o, type, "long", width, saturate: true)
              self.conversion(o, type, "ulong", width)
              self.conversion(o, type, "ulong", width, saturate: true)
              self.conversion(o, type, "double", width)

              # Swizzles

              case width
              when 2
                o.puts("#[inline]", pad: true)
                o.block("pub fn lo(self) -> #{scalar}") do |o|
                  o.puts("return self.0;")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn hi(self) -> #{scalar}") do |o|
                  o.puts("return self.1;")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn odd(self) -> #{scalar}") do |o|
                  o.puts("return self.1;")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn even(self) -> #{scalar}") do |o|
                  o.puts("return self.0;")
                end
              when 3
                o.puts("#[inline]", pad: true)
                o.block("pub fn lo(self) -> #{type}2") do |o|
                  o.puts("return #{type}2(self.0, self.1);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn hi(self) -> #{type}2") do |o|
                  o.puts("return #{type}2(self.2, 0#{".0" if kind.include?(:float)});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn odd(self) -> #{type}2") do |o|
                  o.puts("return #{type}2(self.1, 0#{".0" if kind.include?(:float)});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn even(self) -> #{type}2") do |o|
                  o.puts("return #{type}2(self.0, self.2);")
                end
              else
                o.puts("#[inline]", pad: true)
                o.block("pub fn lo(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{i}"}.join(", ")});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn hi(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{width / 2 + i}"}.join(", ")});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn odd(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{2 * i + 1}"}.join(", ")});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn even(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{2 * i}"}.join(", ")});")
                end
              end
            end
          end

          files << ["#{path}/type_#{name}.rs", io.string]
        end
      end

      TYPES.select { |attributes| attributes[:max_matrix_size] }.each do |attributes|
        scalar = attributes.fetch(:type)
        type = attributes.fetch(:opencl)
        kind = attributes.fetch(:kind)

        max_matrix_size = attributes.fetch(:max_matrix_size)

        (2 .. max_matrix_size).each do |i|
          vector_name = "#{type}#{i}"

          (2 .. max_matrix_size).each do |j|
            io = StringIO.new
            o = Bridge::Output.new(io)

            name = "#{type}#{j}x#{i}"
            content = (["pub #{vector_name}"] * j).join(", ")

            o.puts("use std;", pad: true)
            o.puts("use ::*;")
            o.puts("use ::simd::*;") if i == j

            o.puts("#[repr(C)]", pad: true)
            o.puts("#[derive(Copy, Clone, Debug)]")
            o.puts("pub struct #{name}(#{content});")

            if i == j
              if ["f32", "f64"].include?(scalar)
                typecode = { "f32" => "f", "f64" => "d" }.fetch(scalar)

                o.block("extern", pad: true) do
                  o.puts("fn __invert_#{typecode}#{i}(a: #{name}) -> #{name};")
                end
              end
            end

            o.block("impl std::ops::Add for #{name}", pad: true) do |o|
              o.puts("type Output = Self;")
              o.puts
              o.puts("#[inline]")
              o.block("fn add(self, other: Self) -> Self") do |o|
                o.puts("return #{name}::add(self, other);")
              end
            end

            o.block("impl std::ops::Sub for #{name}", pad: true) do |o|
              o.puts("type Output = Self;")
              o.puts
              o.puts("#[inline]")
              o.block("fn sub(self, other: Self) -> Self") do |o|
                o.puts("return #{name}::sub(self, other);")
              end
            end

            if kind.include?(:float) && i == j # TODO: Implement this for i != j
              o.block("impl std::ops::Mul for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn mul(self, other: Self) -> Self") do |o|
                  o.puts("return self.dot(other);")
                end
              end

              o.block("impl std::ops::Mul<#{vector_name}> for #{name}", pad: true) do |o|
                o.puts("type Output = #{vector_name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn mul(self, other: #{vector_name}) -> #{vector_name}") do |o|
                  o.puts("return self.dot(other);")
                end
              end
            end

            o.block("impl std::ops::Mul<#{scalar}> for #{name}", pad: true) do |o|
              o.puts("type Output = Self;")
              o.puts
              o.puts("#[inline]")
              o.block("fn mul(self, other: #{scalar}) -> Self") do |o|
                o.puts("return #{name}::scale(other, self);")
              end
            end

            if kind.include?(:float) && i == j
              o.block("impl simd::Dot for #{name}", pad: true) do |o|
                o.puts("type Output = #{name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn dot(self, other: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}(#{j.times.map { |k| "self.dot(other.#{k})" }.join(", ")});")
                end
              end

              o.block("impl simd::Dot<#{vector_name}> for #{name}", pad: true) do |o|
                o.puts("type Output = #{vector_name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn dot(self, other: #{vector_name}) -> #{vector_name}") do |o|
                  o.puts("return #{j.times.map { |k| "self.#{k} * other.#{k}" }.join(" + ")};")
                end
              end
            end

            o.block("impl #{name}", pad: true) do
              if i == j
                o.puts("#[inline]", pad: true)
                o.block("pub fn identity(self) -> #{name}") do |o|
                  identity = j.times.map { |k| "#{vector_name}(#{([0.0] * i).tap { |ary| ary[k] = 1.0 }.join(", ")})" }.join(", ")

                  o.puts("return #{name}(#{identity});")
                end
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn scale(a: #{scalar}, x: #{name}) -> #{name}") do |o|
                o.puts("let a = #{vector_name}::broadcast(a);")
                o.puts
                o.puts("return #{name}(#{j.times.map { |k| "a * x.#{k}" }.join(", ")});")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn linear_combination(a: #{scalar}, x: #{name}, b: #{scalar}, y: #{name}) -> #{name}") do |o|
                o.puts("let a = #{vector_name}::broadcast(a);")
                o.puts("let b = #{vector_name}::broadcast(b);")

                o.puts("return #{name}(#{j.times.map { |k| "a * x.#{k} + b * y.#{k}" }.join(", ")});")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn add(x: #{name}, y: #{name}) -> #{name}") do |o|
                o.puts("return #{name}(#{j.times.map { |k| "x.#{k} + y.#{k}" }.join(", ")});")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn sub(x: #{name}, y: #{name}) -> #{name}") do |o|
                o.puts("return #{name}(#{j.times.map { |k| "x.#{k} - y.#{k}" }.join(", ")});")
              end

              transpose_vector_name = "#{type}#{j}"
              transpose_matrix_name = "#{type}#{i}x#{j}"

              o.puts("#[inline]", pad: true)
              o.block("pub fn transpose(self) -> #{transpose_matrix_name}") do |o|
                i.times do |k|
                  o.puts("let c#{k} = #{transpose_vector_name}(#{j.times.map { |l| "(self.#{l}).#{k}" }.join(", ")});")
                end
                o.puts
                o.puts("return #{transpose_matrix_name}(#{i.times.map { |k| "c#{k}" }.join(", ")});")
              end

              # TODO: matrix_determinant

              if i == j && ["f32", "f64"].include?(scalar)
                typecode = { "f32" => "f", "f64" => "d" }.fetch(scalar)

                o.puts("#[inline]", pad: true)
                o.block("pub fn inverse(self) -> #{name}") do |o|
                  o.puts("return unsafe { __invert_#{typecode}#{i}(self) };")
                end
              end

              # matrix_multiply is expressed via the `simd::Dot` trait

              # TODO: o.puts("#[inline]", pad: true)
              #o.block("pub fn equal(x: #{name}, y: #{name}) -> bool") do |o|
              #end

              # TODO: o.puts("#[inline]", pad: true)
              # o.block("pub fn almost_equal_elements(x: #{name}, y: #{name}, tolerance: #{scalar}) -> bool") do |o|
              # end

              # TODO: o.puts("#[inline]", pad: true)
              # o.block("pub fn almost_equal_elements_relative(x: #{name}, y: #{name}, tolerance: #{scalar}) -> bool") do |o|
              # end
            end

            files << ["#{path}/type_#{name}.rs", io.string]
          end
        end
      end

      files
    end
    
    def self.conversion(o, in_ty, out_ty, width, saturate: false)
      in_ty = TYPES_BY_NAME.fetch(in_ty)
      out_ty = TYPES_BY_NAME.fetch(out_ty)

      in_scalar = "#{in_ty.fetch(:type)}"
      in_type = "#{in_ty.fetch(:opencl)}"
      in_name = "#{in_type}#{width}"
      in_size = in_ty.fetch(:size)
      in_kind = in_ty.fetch(:kind)

      out_scalar = "#{out_ty.fetch(:type)}"
      out_type = "#{out_ty.fetch(:opencl)}"
      out_name = "#{out_type}#{width}"
      out_size = out_ty.fetch(:size)
      out_kind = out_ty.fetch(:kind)

      o.puts("#[inline]", pad: true)
      o.block("pub fn to_#{out_type}#{"_sat" if saturate}(x: #{in_name}) -> #{out_name}") do |o|
        if saturate
          min = "#{in_name}::broadcast(std::#{out_scalar}::MIN as #{in_scalar})"
          max = "#{in_name}::broadcast(std::#{out_scalar}::MAX as #{in_scalar})"

          if in_scalar == out_scalar
            o.puts("return x;")
          elsif in_kind == out_kind && in_size < out_size
            o.puts("return #{in_name}::to_#{out_type}(x);")
          elsif in_kind.include?(:signed) && out_kind.include?(:unsigned) && in_size <= out_size
            o.puts("return #{in_name}::to_#{out_type}(simd::max(x, #{in_name}::broadcast(0)));")
          elsif in_kind.include?(:unsigned)
            o.puts("return #{in_name}::to_#{out_type}(simd::min(x, #{max}));")
          else
            o.puts("return #{in_name}::to_#{out_type}(simd::clamp(x, #{min}, #{max}));")
          end
        else
          if width == 3 && !in_kind.include?(:float) && in_size < out_size # TODO: Fix this compiler bug
            o.puts("return #{out_name}(x.0 as #{out_scalar}, x.1 as #{out_scalar}, x.2 as #{out_scalar});")
          else
            o.puts("return unsafe { simd_cast(x) };")
          end
        end
      end
    end
  end
end
