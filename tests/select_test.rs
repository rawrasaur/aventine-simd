#![allow(overflowing_literals)]

extern crate hagane_simd as simd;

use simd::*;

#[test]
fn test_select() {
  assert_eq!(select(int2(0x80000000, 0x00000000), float2(10.0, -2.0), float2(11.0, -3.0)), float2(11.0, -2.0));
  assert_eq!(select(int3(0x80000000, 0x00000000, 0x00000000), float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0)), float3(11.0, -2.0, -9.5));
  assert_eq!(select(int4(0x80000000, 0x00000000, 0x00000000, 0x80000000), float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0)), float4(11.0, -2.0, -9.5, 0.0));

  assert_eq!(select(long2(0x8000000000000000, 0x0000000000000000), double2(10.0, -2.0), double2(11.0, -3.0)), double2(11.0, -2.0));
  assert_eq!(select(long3(0x8000000000000000, 0x0000000000000000, 0x0000000000000000), double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0)), double3(11.0, -2.0, -9.5));
  assert_eq!(select(long4(0x8000000000000000, 0x0000000000000000, 0x0000000000000000, 0x800000000000FFFF), double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0)), double4(11.0, -2.0, -9.5, 0.0));
}

#[test]
fn test_bitselect() {
  assert_eq!(bitselect(char2(0x11, 0x33), char2(0xFF, 0x00), char2(0x00, 0xFF)), char2(0xEE, 0x33));
  assert_eq!(bitselect(char3(0x11, 0x33, 0x55), char3(0xFF, 0x00, 0x00), char3(0x00, 0xFF, 0xFF)), char3(0xEE, 0x33, 0x55));
  assert_eq!(bitselect(char4(0x11, 0x33, 0x55, 0x77), char4(0xFF, 0x00, 0x00, 0xFF), char4(0x00, 0xFF, 0xFF, 0x00)), char4(0xEE, 0x33, 0x55, 0x88));

  assert_eq!(bitselect(char2(0x11, 0x33), uchar2(0xFF, 0x00), uchar2(0x00, 0xFF)), uchar2(0xEE, 0x33));
  assert_eq!(bitselect(char3(0x11, 0x33, 0x55), uchar3(0xFF, 0x00, 0x00), uchar3(0x00, 0xFF, 0xFF)), uchar3(0xEE, 0x33, 0x55));
  assert_eq!(bitselect(char4(0x11, 0x33, 0x55, 0x77), uchar4(0xFF, 0x00, 0x00, 0xFF), uchar4(0x00, 0xFF, 0xFF, 0x00)), uchar4(0xEE, 0x33, 0x55, 0x88));

  assert_eq!(bitselect(short2(0x1111, 0x3333), short2(0xFFFF, 0x0000), short2(0x0000, 0xFFFF)), short2(0xEEEE, 0x3333));
  assert_eq!(bitselect(short3(0x1111, 0x3333, 0x5555), short3(0xFFFF, 0x0000, 0x0000), short3(0x0000, 0xFFFF, 0xFFFF)), short3(0xEEEE, 0x3333, 0x5555));
  assert_eq!(bitselect(short4(0x1111, 0x3333, 0x5555, 0x7777), short4(0xFFFF, 0x0000, 0x0000, 0xFFFF), short4(0x0000, 0xFFFF, 0xFFFF, 0x0000)), short4(0xEEEE, 0x3333, 0x5555, 0x8888));

  assert_eq!(bitselect(short2(0x1111, 0x3333), ushort2(0xFFFF, 0x0000), ushort2(0x0000, 0xFFFF)), ushort2(0xEEEE, 0x3333));
  assert_eq!(bitselect(short3(0x1111, 0x3333, 0x5555), ushort3(0xFFFF, 0x0000, 0x0000), ushort3(0x0000, 0xFFFF, 0xFFFF)), ushort3(0xEEEE, 0x3333, 0x5555));
  assert_eq!(bitselect(short4(0x1111, 0x3333, 0x5555, 0x7777), ushort4(0xFFFF, 0x0000, 0x0000, 0xFFFF), ushort4(0x0000, 0xFFFF, 0xFFFF, 0x0000)), ushort4(0xEEEE, 0x3333, 0x5555, 0x8888));

  assert_eq!(bitselect(int2(0x11111111, 0x33333333), int2(0xFFFFFFFF, 0x00000000), int2(0x00000000, 0xFFFFFFFF)), int2(0xEEEEEEEE, 0x33333333));
  assert_eq!(bitselect(int3(0x11111111, 0x33333333, 0x55555555), int3(0xFFFFFFFF, 0x00000000, 0x00000000), int3(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF)), int3(0xEEEEEEEE, 0x33333333, 0x55555555));
  assert_eq!(bitselect(int4(0x11111111, 0x33333333, 0x55555555, 0x77777777), int4(0xFFFFFFFF, 0x00000000, 0x00000000, 0xFFFFFFFF), int4(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000)), int4(0xEEEEEEEE, 0x33333333, 0x55555555, 0x88888888));

  assert_eq!(bitselect(int2(0x11111111, 0x33333333), uint2(0xFFFFFFFF, 0x00000000), uint2(0x00000000, 0xFFFFFFFF)), uint2(0xEEEEEEEE, 0x33333333));
  assert_eq!(bitselect(int3(0x11111111, 0x33333333, 0x55555555), uint3(0xFFFFFFFF, 0x00000000, 0x00000000), uint3(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF)), uint3(0xEEEEEEEE, 0x33333333, 0x55555555));
  assert_eq!(bitselect(int4(0x11111111, 0x33333333, 0x55555555, 0x77777777), uint4(0xFFFFFFFF, 0x00000000, 0x00000000, 0xFFFFFFFF), uint4(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000)), uint4(0xEEEEEEEE, 0x33333333, 0x55555555, 0x88888888));

  assert_eq!(bitselect(long2(0x1111111100000000, 0x3333333300000000), long2(0xFFFFFFFF00000000, 0x0000000000000000), long2(0x0000000000000000, 0xFFFFFFFF00000000)), long2(0xEEEEEEEE00000000, 0x3333333300000000));
  assert_eq!(bitselect(long3(0x1111111100000000, 0x3333333300000000, 0x5555555500000000), long3(0xFFFFFFFF00000000, 0x0000000000000000, 0x0000000000000000), long3(0x0000000000000000, 0xFFFFFFFF00000000, 0xFFFFFFFF00000000)), long3(0xEEEEEEEE00000000, 0x3333333300000000, 0x5555555500000000));
  assert_eq!(bitselect(long4(0x1111111100000000, 0x3333333300000000, 0x5555555500000000, 0x7777777700000000), long4(0xFFFFFFFF00000000, 0x0000000000000000, 0x0000000000000000, 0xFFFFFFFF00000000), long4(0x0000000000000000, 0xFFFFFFFF00000000, 0xFFFFFFFF00000000, 0x0000000000000000)), long4(0xEEEEEEEE00000000, 0x3333333300000000, 0x5555555500000000, 0x8888888800000000));

  assert_eq!(bitselect(long2(0x1111111100000000, 0x3333333300000000), ulong2(0xFFFFFFFF00000000, 0x0000000000000000), ulong2(0x0000000000000000, 0xFFFFFFFF00000000)), ulong2(0xEEEEEEEE00000000, 0x3333333300000000));
  assert_eq!(bitselect(long3(0x1111111100000000, 0x3333333300000000, 0x5555555500000000), ulong3(0xFFFFFFFF00000000, 0x0000000000000000, 0x0000000000000000), ulong3(0x0000000000000000, 0xFFFFFFFF00000000, 0xFFFFFFFF00000000)), ulong3(0xEEEEEEEE00000000, 0x3333333300000000, 0x5555555500000000));
  assert_eq!(bitselect(long4(0x1111111100000000, 0x3333333300000000, 0x5555555500000000, 0x7777777700000000), ulong4(0xFFFFFFFF00000000, 0x0000000000000000, 0x0000000000000000, 0xFFFFFFFF00000000), ulong4(0x0000000000000000, 0xFFFFFFFF00000000, 0xFFFFFFFF00000000, 0x0000000000000000)), ulong4(0xEEEEEEEE00000000, 0x3333333300000000, 0x5555555500000000, 0x8888888800000000));
}
