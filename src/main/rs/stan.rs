#pragma version(1)
#pragma rs java_package_name(org.jcodec.android)
#pragma rs_fp_relaxed

rs_allocation reference;

#pragma rs reduce(findFull) \
  accumulator(findFullAccum) \
  combiner(findCombine) \
  outconverter(findOut)

typedef ushort FindAccumType[16];

static void findFullAccum(FindAccumType* accum, uchar val, uint32_t x, uint32_t y) {
  for (int i = 0; i < 16; i++) {
    int _sx    = i & 0x3;
    int _sy    = i >> 2;
    uchar refChar = rsGetElementAt_uchar(reference, _sx + x, _sy + y);
    (*accum)[i] += abs(val - refChar);
  }
}

static void findCombine(FindAccumType* accum, const FindAccumType* other) {
  for (int i = 0; i < 16; i++) {
    (*accum)[i] += (*other)[i];
  }
}

static void findOut(int* result, const FindAccumType* accum) {
  int minIdx = 0;
  for (int i = 0; i < 16; i++) {
    if ((*accum)[i] < (*accum)[minIdx])
      minIdx = i;
  }
  *result = minIdx;
}

#define POS_00 0
#define POS_10 1
#define POS_20 2
#define POS_30 3
#define POS_01 4
#define POS_11 5
#define POS_21 6
#define POS_31 7
#define POS_02 8
#define POS_12 9
#define POS_22 10
#define POS_32 11
#define POS_03 12
#define POS_13 13
#define POS_23 14
#define POS_33 15

#pragma rs reduce(findSub) \
  accumulator(findSubAccum) \
  combiner(findCombine) \
  outconverter(findOut)

static void findSubAccum(FindAccumType* accum, uchar val, uint32_t x, uint32_t y) {
  uchar ref00 = rsGetElementAt_uchar(reference, x+2, y+2);
  uchar ref40 = rsGetElementAt_uchar(reference, x+3, y+2);
  uchar ref04 = rsGetElementAt_uchar(reference, x+2, y+3);
  uchar ref44 = rsGetElementAt_uchar(reference, x+3, y+3);
  short tmp0 =
     rsGetElementAt_uchar(reference,   x, y  )
    -rsGetElementAt_uchar(reference, x+1, y  ) *5
    +rsGetElementAt_uchar(reference, x+2, y  ) *20
    +rsGetElementAt_uchar(reference, x+3, y  ) *20
    -rsGetElementAt_uchar(reference, x+4, y  ) *5
    +rsGetElementAt_uchar(reference, x+5, y  )
    ;
  short tmp1 =
     rsGetElementAt_uchar(reference,   x, y+1)
    -rsGetElementAt_uchar(reference, x+1, y+1) *5
    +rsGetElementAt_uchar(reference, x+2, y+1) *20
    +rsGetElementAt_uchar(reference, x+3, y+1) *20
    -rsGetElementAt_uchar(reference, x+4, y+1) *5
    +rsGetElementAt_uchar(reference, x+5, y+1)
    ;
  short tmp2 =
     rsGetElementAt_uchar(reference,   x, y+2)
    -rsGetElementAt_uchar(reference, x+1, y+2) *5
    +rsGetElementAt_uchar(reference, x+2, y+2) *20
    +rsGetElementAt_uchar(reference, x+3, y+2) *20
    -rsGetElementAt_uchar(reference, x+4, y+2) *5
    +rsGetElementAt_uchar(reference, x+5, y+2)
    ;
  short tmp3 =
     rsGetElementAt_uchar(reference,   x, y+3)
    -rsGetElementAt_uchar(reference, x+1, y+3) *5
    +rsGetElementAt_uchar(reference, x+2, y+3) *20
    +rsGetElementAt_uchar(reference, x+3, y+3) *20
    -rsGetElementAt_uchar(reference, x+4, y+3) *5
    +rsGetElementAt_uchar(reference, x+5, y+3)
    ;
  short tmp4 =
     rsGetElementAt_uchar(reference,   x, y+4)
    -rsGetElementAt_uchar(reference, x+1, y+4) *5
    +rsGetElementAt_uchar(reference, x+2, y+4) *20
    +rsGetElementAt_uchar(reference, x+3, y+4) *20
    -rsGetElementAt_uchar(reference, x+4, y+4) *5
    +rsGetElementAt_uchar(reference, x+5, y+4)
    ;
  short tmp5 =
     rsGetElementAt_uchar(reference,   x, y+5)
    -rsGetElementAt_uchar(reference, x+1, y+5) *5
    +rsGetElementAt_uchar(reference, x+2, y+5) *20
    +rsGetElementAt_uchar(reference, x+3, y+5) *20
    -rsGetElementAt_uchar(reference, x+4, y+5) *5
    +rsGetElementAt_uchar(reference, x+5, y+5)
    ;
  uchar ref20 = clamp((tmp2+16) >> 5, 0, 255);
  uchar ref24 = clamp((tmp3+16) >> 5, 0, 255);
  uchar ref02 = clamp((
     rsGetElementAt_uchar(reference, x+2,   y)
    -rsGetElementAt_uchar(reference, x+2, y+1) *5
    +rsGetElementAt_uchar(reference, x+2, y+2) *20
    +rsGetElementAt_uchar(reference, x+2, y+3) *20
    -rsGetElementAt_uchar(reference, x+2, y+4) *5
    +rsGetElementAt_uchar(reference, x+2, y+5)
    +16) >> 5, 0, 255);
  uchar ref42 = clamp((
     rsGetElementAt_uchar(reference, x+3,   y)
    -rsGetElementAt_uchar(reference, x+3, y+1) *5
    +rsGetElementAt_uchar(reference, x+3, y+2) *20
    +rsGetElementAt_uchar(reference, x+3, y+3) *20
    -rsGetElementAt_uchar(reference, x+3, y+4) *5
    +rsGetElementAt_uchar(reference, x+3, y+5)
    +16) >> 5, 0, 255);
  uchar ref22 = clamp((
     tmp0
    -tmp1 *5
    +tmp2 *20
    +tmp3 *20
    -tmp4 *5
    +tmp5
    +512) >> 10, 0, 255);
  uchar ref10 = (ref00 + ref20 + 1) >> 1;
  uchar ref30 = (ref40 + ref20 + 1) >> 1;
  uchar ref01 = (ref00 + ref02 + 1) >> 1;
  uchar ref11 = (ref00 + ref22 + 1) >> 1;
  uchar ref21 = (ref20 + ref22 + 1) >> 1;
  uchar ref31 = (ref40 + ref22 + 1) >> 1;
  uchar ref12 = (ref02 + ref22 + 1) >> 1;
  uchar ref32 = (ref42 + ref22 + 1) >> 1;
  uchar ref03 = (ref04 + ref02 + 1) >> 1;
  uchar ref13 = (ref04 + ref22 + 1) >> 1;
  uchar ref23 = (ref24 + ref22 + 1) >> 1;
  uchar ref33 = (ref44 + ref22 + 1) >> 1;
  (*accum)[POS_00] += abs(val - ref00);
  (*accum)[POS_10] += abs(val - ref10);
  (*accum)[POS_20] += abs(val - ref20);
  (*accum)[POS_30] += abs(val - ref30);
  (*accum)[POS_01] += abs(val - ref01);
  (*accum)[POS_11] += abs(val - ref11);
  (*accum)[POS_21] += abs(val - ref21);
  (*accum)[POS_31] += abs(val - ref31);
  (*accum)[POS_02] += abs(val - ref02);
  (*accum)[POS_12] += abs(val - ref12);
  (*accum)[POS_22] += abs(val - ref22);
  (*accum)[POS_32] += abs(val - ref32);
  (*accum)[POS_03] += abs(val - ref03);
  (*accum)[POS_13] += abs(val - ref13);
  (*accum)[POS_23] += abs(val - ref23);
  (*accum)[POS_33] += abs(val - ref33);
}

uchar RS_KERNEL buildIndex(uchar dummy, uint32_t x, uint32_t y)
{
    ushort sum = 0;
    for (int i = 0; i < 4; i++) {
      for (int j = 0; j < 4; j++) {
        sum += rsGetElementAt_uchar(reference, (x << 2) + j, (y << 2) + i);
      }
    }
    return (uchar)(sum >> 4);
}

uchar RS_KERNEL toGrey(uchar4 src)
{
    //0.299f, 0.587f, 0.114f
    return (
    src.x * 306 +
    src.y * 601 +
    src.z * 117) >> 10;
}