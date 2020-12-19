#pragma version(1)
#pragma rs java_package_name(org.jcodec.android)
#pragma rs_fp_relaxed

rs_allocation BuildIdxRef;
uchar RS_KERNEL buildIdx(uchar dummy, uint32_t x, uint32_t y)
{
    ushort sum = 0;
    for (int i = 0; i < 4; i++) {
      for (int j = 0; j < 4; j++) {
        sum += rsGetElementAt_uchar(BuildIdxRef, (x << 2) + j, (y << 2) + i);
      }
    }
    return (uchar)(sum >> 4);
}

rs_allocation CalcDistSrc;
rs_allocation CalcDistRef;
ushort CalcDistStartX;
ushort CalcDistStartY;
ushort RS_KERNEL calcDist(uint32_t x, uint32_t y) {
  ushort dist = 0;
  ushort refBaseX = x - 4 + CalcDistStartX;
  ushort refBaseY = y - 4 + CalcDistStartY;
  for (int i = 0; i < 16; i++) {
    uchar ox = i & 0x3;
    uchar oy = i >> 2;
    ushort refX = clamp((short)(refBaseX + ox), (short)0, (short)(rsAllocationGetDimX(CalcDistRef) - 1));
    ushort refY = clamp((short)(refBaseY + oy), (short)0, (short)(rsAllocationGetDimY(CalcDistRef) - 1));
    dist += abs(rsGetElementAt_uchar(CalcDistSrc, CalcDistStartX + ox, CalcDistStartY + oy) -
                rsGetElementAt_uchar(CalcDistRef, refX, refY));
  }
  return dist;
}

rs_allocation ReduceDistArg;
ushort2 RS_KERNEL reduceDist(uint32_t x) {
  ushort minDist = 65535;
  ushort bestMagn = 65535;
  ushort bestY = 0;
  for (int i = 0; i < 9; i++) {
    ushort magn = abs(i - 4);
    ushort dist = rsGetElementAt_ushort(ReduceDistArg, x, i);
    if (minDist > dist || (dist == minDist && magn < bestMagn)) {
      minDist = dist;
      bestMagn = magn;
      bestY = i;
    }
  }
  ushort2 ret = {minDist, bestY};
  return ret;
}

static ushort3 minDist1D(rs_allocation dist1D) {
    ushort minDist = 65535;
    int bestX = 0;
    int bestY = 0;
    ushort bestMagn = 65535;
    for (int i = 0; i < 9; i++) {
        ushort2 dist = rsGetElementAt_ushort2(dist1D, i);
        ushort magn = abs(i - 4) + abs(dist.y - 4);
        if (dist.x < minDist || (dist.x == minDist && magn < bestMagn)) {
          minDist = dist.x;
          bestX = i;
          bestY = dist.y;
          bestMagn = magn;
      }
    }
    ushort3 ret = {minDist, bestX, bestY};
    return ret;
}

static ushort3 minDist2D(rs_allocation dist2D) {
    rs_allocation dist1D = rsCreateAllocation_ushort2(9);

    ReduceDistArg = dist2D;
    rsForEach(reduceDist, dist1D);

    ushort3 minDist = minDist1D(dist1D);

    rsClearObject(&dist1D);

    return minDist;
}

rs_allocation FindFullSrc;
rs_allocation FindFullRef;
ushort FindFullSrcX;
ushort FindFullSrcY;
short FindFullRefX;
short FindFullRefY;
ushort RS_KERNEL findFull(uint32_t x, uint32_t y) {
  ushort refBaseX = x - 4 + FindFullRefX;
  ushort refBaseY = y - 4 + FindFullRefY;
  ushort dist = 0;
  for (int i = 0; i < 256; i++) {
    int ox    = i & 0xf;
    int oy    = i >> 4;

    ushort refX = clamp((short)(refBaseX + ox), (short)0, (short)(rsAllocationGetDimX(FindFullRef) - 1));
    ushort refY = clamp((short)(refBaseY + oy), (short)0, (short)(rsAllocationGetDimY(FindFullRef) - 1));

    uchar refPix = rsGetElementAt_uchar(FindFullRef, refX, refY);
    uchar srcPix = rsGetElementAt_uchar(FindFullSrc, ox + FindFullSrcX, oy + FindFullSrcY);
    dist += abs(srcPix - refPix);
  }
  return dist;
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

rs_allocation FindSubSrc;
rs_allocation FindSubRef;
rs_allocation FindSubDist;
ushort FindSubSrcX;
ushort FindSubSrcY;
short FindSubRefX;
short FindSubRefY;
ushort RS_KERNEL findSub(uint32_t x, uint32_t y) {
  ushort x0 = clamp((short)(x + FindSubRefX - 2), (short)0, (short)(rsAllocationGetDimX(FindSubRef) - 1));
  ushort x1 = clamp((short)(x + FindSubRefX - 1), (short)0, (short)(rsAllocationGetDimX(FindSubRef) - 1));
  ushort x2 = clamp((short)(x + FindSubRefX    ), (short)0, (short)(rsAllocationGetDimX(FindSubRef) - 1));
  ushort x3 = clamp((short)(x + FindSubRefX + 1), (short)0, (short)(rsAllocationGetDimX(FindSubRef) - 1));
  ushort x4 = clamp((short)(x + FindSubRefX + 2), (short)0, (short)(rsAllocationGetDimX(FindSubRef) - 1));
  ushort x5 = clamp((short)(x + FindSubRefX + 3), (short)0, (short)(rsAllocationGetDimX(FindSubRef) - 1));

  ushort y0 = clamp((short)(y + FindSubRefY - 2), (short)0, (short)(rsAllocationGetDimY(FindSubRef) - 1));
  ushort y1 = clamp((short)(y + FindSubRefY - 1), (short)0, (short)(rsAllocationGetDimY(FindSubRef) - 1));
  ushort y2 = clamp((short)(y + FindSubRefY    ), (short)0, (short)(rsAllocationGetDimY(FindSubRef) - 1));
  ushort y3 = clamp((short)(y + FindSubRefY + 1), (short)0, (short)(rsAllocationGetDimY(FindSubRef) - 1));
  ushort y4 = clamp((short)(y + FindSubRefY + 2), (short)0, (short)(rsAllocationGetDimY(FindSubRef) - 1));
  ushort y5 = clamp((short)(y + FindSubRefY + 3), (short)0, (short)(rsAllocationGetDimY(FindSubRef) - 1));

  uchar ref00 = rsGetElementAt_uchar(FindSubRef, x2, y2);
  uchar ref40 = rsGetElementAt_uchar(FindSubRef, x3, y2);
  uchar ref04 = rsGetElementAt_uchar(FindSubRef, x2, y3);
  uchar ref44 = rsGetElementAt_uchar(FindSubRef, x3, y3);
  short tmp0 =
     rsGetElementAt_uchar(FindSubRef, x0, y0  )
    -rsGetElementAt_uchar(FindSubRef, x1, y0  ) *5
    +rsGetElementAt_uchar(FindSubRef, x2, y0  ) *20
    +rsGetElementAt_uchar(FindSubRef, x3, y0  ) *20
    -rsGetElementAt_uchar(FindSubRef, x4, y0  ) *5
    +rsGetElementAt_uchar(FindSubRef, x5, y0  )
    ;
  short tmp1 =
     rsGetElementAt_uchar(FindSubRef, x0, y1)
    -rsGetElementAt_uchar(FindSubRef, x1, y1) *5
    +rsGetElementAt_uchar(FindSubRef, x2, y1) *20
    +rsGetElementAt_uchar(FindSubRef, x3, y1) *20
    -rsGetElementAt_uchar(FindSubRef, x4, y1) *5
    +rsGetElementAt_uchar(FindSubRef, x5, y1)
    ;
  short tmp2 =
     rsGetElementAt_uchar(FindSubRef, x0, y2)
    -rsGetElementAt_uchar(FindSubRef, x1, y2) *5
    +rsGetElementAt_uchar(FindSubRef, x2, y2) *20
    +rsGetElementAt_uchar(FindSubRef, x3, y2) *20
    -rsGetElementAt_uchar(FindSubRef, x4, y2) *5
    +rsGetElementAt_uchar(FindSubRef, x5, y2)
    ;
  short tmp3 =
     rsGetElementAt_uchar(FindSubRef, x0, y3)
    -rsGetElementAt_uchar(FindSubRef, x1, y3) *5
    +rsGetElementAt_uchar(FindSubRef, x2, y3) *20
    +rsGetElementAt_uchar(FindSubRef, x3, y3) *20
    -rsGetElementAt_uchar(FindSubRef, x4, y3) *5
    +rsGetElementAt_uchar(FindSubRef, x5, y3)
    ;
  short tmp4 =
     rsGetElementAt_uchar(FindSubRef, x0, y4)
    -rsGetElementAt_uchar(FindSubRef, x1, y4) *5
    +rsGetElementAt_uchar(FindSubRef, x2, y4) *20
    +rsGetElementAt_uchar(FindSubRef, x3, y4) *20
    -rsGetElementAt_uchar(FindSubRef, x4, y4) *5
    +rsGetElementAt_uchar(FindSubRef, x5, y4)
    ;
  short tmp5 =
     rsGetElementAt_uchar(FindSubRef, x0, y5)
    -rsGetElementAt_uchar(FindSubRef, x1, y5) *5
    +rsGetElementAt_uchar(FindSubRef, x2, y5) *20
    +rsGetElementAt_uchar(FindSubRef, x3, y5) *20
    -rsGetElementAt_uchar(FindSubRef, x4, y5) *5
    +rsGetElementAt_uchar(FindSubRef, x5, y5)
    ;
  uchar ref20 = clamp((tmp2+16) >> 5, 0, 255);
  uchar ref24 = clamp((tmp3+16) >> 5, 0, 255);
  uchar ref02 = clamp((
     rsGetElementAt_uchar(FindSubRef, x2, y0)
    -rsGetElementAt_uchar(FindSubRef, x2, y1) *5
    +rsGetElementAt_uchar(FindSubRef, x2, y2) *20
    +rsGetElementAt_uchar(FindSubRef, x2, y3) *20
    -rsGetElementAt_uchar(FindSubRef, x2, y4) *5
    +rsGetElementAt_uchar(FindSubRef, x2, y5)
    +16) >> 5, 0, 255);
  uchar ref42 = clamp((
     rsGetElementAt_uchar(FindSubRef, x3, y0)
    -rsGetElementAt_uchar(FindSubRef, x3, y1) *5
    +rsGetElementAt_uchar(FindSubRef, x3, y2) *20
    +rsGetElementAt_uchar(FindSubRef, x3, y3) *20
    -rsGetElementAt_uchar(FindSubRef, x3, y4) *5
    +rsGetElementAt_uchar(FindSubRef, x3, y5)
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

  uchar val = rsGetElementAt_uchar(FindSubSrc, FindSubSrcX + x, FindSubSrcY + y);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref00), x, y, POS_00);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref10), x, y, POS_10);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref20), x, y, POS_20);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref30), x, y, POS_30);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref01), x, y, POS_01);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref11), x, y, POS_11);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref21), x, y, POS_21);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref31), x, y, POS_31);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref02), x, y, POS_02);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref12), x, y, POS_12);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref22), x, y, POS_22);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref32), x, y, POS_32);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref03), x, y, POS_03);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref13), x, y, POS_13);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref23), x, y, POS_23);
  rsSetElementAt_ushort(FindSubDist, abs(val - ref33), x, y, POS_33);

  return 0;
}

ushort RS_KERNEL findSubSum(uint32_t x) {
  ushort dist = 0;
  for (int i = 0; i < 16; i++) {
    for (int j = 0; j < 16; j++) {
      dist += rsGetElementAt_ushort(FindSubDist, j, i, x);
    }
  }
  return dist;
}

static ushort3 min16(rs_allocation dist1D) {
    ushort minDist = 0xffff;
    ushort3 subMv = {0, 0, 0};
    for (int i = 0; i < 16; i++) {
      ushort dist = rsGetElementAt_ushort(dist1D, i);
      if (dist < minDist) {
        minDist = dist;
        subMv.x = minDist;
        subMv.y = i & 0x3;
        subMv.z = i >> 2;
      }
    }
    return subMv;
}

void calcMotion(rs_allocation srcIdx, rs_allocation refIdx, rs_allocation src, rs_allocation ref,
                int mbX, int mbY) {
    rs_allocation dist2D = rsCreateAllocation_ushort(9,9);

    CalcDistSrc = srcIdx;
    CalcDistRef = refIdx;
    CalcDistStartX = mbX << 2;
    CalcDistStartY = mbY << 2;

    rsForEach(calcDist, dist2D);

    ushort3 minIdx[4] = {0,0,0,0};
    for (int i = 0; i < 4; i++) {
        minIdx[i] = minDist2D(dist2D);
        rsSetElementAt_ushort(dist2D, 0xffff, minIdx[i].y, minIdx[i].z);
    }

    ushort3 minFull = {0xffff, 0, 0};
    ushort bestMagn = 0xffff;
    short2 idxMv;
    for (int i = 0; i < 4; i++) {
        short2 mv = {
            (minIdx[i].y - 4) << 2,
            (minIdx[i].z - 4) << 2
            };

        FindFullSrc = src;
        FindFullRef = ref;
        FindFullSrcX = mbX << 4;
        FindFullSrcY = mbY << 4;
        FindFullRefX = FindFullSrcX + mv.x;
        FindFullRefY = FindFullSrcY + mv.y;

        rsForEach(findFull, dist2D);
        ushort3 candFull = minDist2D(dist2D);
        ushort magn = abs(mv.x + candFull.y - 4) + abs(mv.y + candFull.z - 4);
        if (candFull.x < minFull.x || (candFull.x == minFull.x && magn < bestMagn)) {
            minFull = candFull;
            idxMv = mv;
            bestMagn = magn;
        }
    }

    short2 fullMv = {
            minFull.y + idxMv.x - 4,
            minFull.z + idxMv.y - 4
            };
    rs_allocation fake2D = rsCreateAllocation_ushort(16,16);
    rs_allocation dist1D = rsCreateAllocation_ushort(16);
    rs_allocation dist3D = rsCreateAllocation_ushort(16,16,16);
    FindSubSrc = src;
    FindSubRef = ref;
    FindSubDist = dist3D;
    FindSubSrcX = mbX << 4;
    FindSubSrcY = mbY << 4;
    FindSubRefX = FindSubSrcX + fullMv.x;
    FindSubRefY = FindSubSrcY + fullMv.y;

    rsForEach(findSub, fake2D);
    rsForEach(findSubSum, dist1D);
    ushort3 subMv = min16(dist1D);

    uint result[] = {(fullMv.x << 2) + subMv.y, (fullMv.y << 2) + subMv.z, subMv.x, mbX, mbY};
    rsSendToClientBlocking(1, &result, sizeof(result));

    rsClearObject(&dist2D);
    rsClearObject(&fake2D);
    rsClearObject(&dist1D);
    rsClearObject(&dist3D);
}

uchar RS_KERNEL toGrey(uchar4 src)
{
    //0.299f, 0.587f, 0.114f
    return (
    src.x * 306 +
    src.y * 601 +
    src.z * 117) >> 10;
}

