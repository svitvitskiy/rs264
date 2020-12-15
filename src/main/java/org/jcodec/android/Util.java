package org.jcodec.android;

import android.app.Activity;
import android.graphics.Bitmap;
import android.renderscript.Allocation;
import android.renderscript.Element;
import android.renderscript.RenderScript;
import android.renderscript.Script;
import android.renderscript.Type;
import android.util.Log;

public class Util {
    private static final String TAG = "Util";

    public static int sum(int a, int b) {
        return a + b;
    }

    public static int findFull(Activity activity, byte[] src, byte[] ref) {
        RenderScript rs = RenderScript.create(activity);
        ScriptC_stan mScript = new ScriptC_stan(rs);
        Allocation srcAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), 4, 4));
        Allocation refAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), 8, 8));

        Allocation outputAlloc = Allocation.createSized(rs, Element.U16(rs), src.length);
        srcAlloc.copyFrom(src);
        refAlloc.copyFrom(ref);

        mScript.set_reference(refAlloc);
        int best = mScript.reduce_findFull(srcAlloc).get();
        Log.i(TAG, "Best pos x=" + (best % 4) + ",y=" + (best / 4));
        return 0;
    }

    public static int findSub(Activity activity, byte[] src, byte[] ref) {
        RenderScript rs = RenderScript.create(activity);
        ScriptC_stan mScript = new ScriptC_stan(rs);
        Allocation srcAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), 4, 4));
        Allocation refAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), 9, 9));

        Allocation outputAlloc = Allocation.createSized(rs, Element.U16(rs), src.length);
        srcAlloc.copyFrom(src);
        refAlloc.copyFrom(ref);

        mScript.set_reference(refAlloc);
        int best = mScript.reduce_findSub(srcAlloc).get();
        Log.i(TAG, "Best pos x=" + (best % 4) + ",y=" + (best / 4));
        return 0;
    }

    public static void findMotion(Activity activity, Bitmap src, Bitmap ref, int mbX, int mbY) {
        RenderScript rs = RenderScript.create(activity);
        Allocation srcRgbAlloc = Allocation.createFromBitmap(rs, src);
        Allocation refRgbAlloc = Allocation.createFromBitmap(rs, ref);

        Allocation srcAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), srcRgbAlloc.getType().getX(), srcRgbAlloc.getType().getY()));
        Allocation refAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), refRgbAlloc.getType().getX(), refRgbAlloc.getType().getY()));

        Allocation srcIdxAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), srcRgbAlloc.getType().getX() >> 2, srcRgbAlloc.getType().getY() >> 2));
        Allocation refIdxAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), refRgbAlloc.getType().getX() >> 2, refRgbAlloc.getType().getY() >> 2));

        ScriptC_stan mScript = new ScriptC_stan(rs);
        mScript.foreach_toGrey(srcRgbAlloc, srcAlloc);
        mScript.foreach_toGrey(refRgbAlloc, refAlloc);

        mScript.set_reference(srcAlloc);
        mScript.forEach_buildIndex(srcIdxAlloc, srcIdxAlloc);
        mScript.set_reference(refAlloc);
        mScript.forEach_buildIndex(refIdxAlloc, refIdxAlloc);

        // Searching in index for 16x16
    }
}
