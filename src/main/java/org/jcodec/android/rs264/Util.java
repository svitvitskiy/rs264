package org.jcodec.android.rs264;

import android.app.Activity;
import android.graphics.Bitmap;
import android.renderscript.Allocation;
import android.renderscript.Element;
import android.renderscript.RenderScript;
import android.renderscript.Script;
import android.renderscript.ScriptGroup;
import android.renderscript.Type;
import android.util.Log;

import org.jcodec.android.ScriptC_rs264;

public class Util {
    private static final String TAG = "Util";

    public static void findMotion(Activity activity, Bitmap src, Bitmap ref) {
        RenderScript rs = RenderScript.create(activity);
        ScriptC_rs264 script = new ScriptC_rs264(rs);
        rs.setMessageHandler(new RenderScript.RSMessageHandler() {
            public void run() {
                if (mID == 1) {
                    int mvXIdx = mData[0];
                    int mvYIdx = mData[1];
                    int dist = mData[2];
                    int mbX = mData[3];
                    int mbY = mData[4];
                    Log.i(TAG, "[" + mbX + "," + mbY + "] best idx: (" + mvXIdx + "," + mvYIdx + "), dist=" + dist);
                }
            }
        });
        Allocation srcRgbAlloc = Allocation.createFromBitmap(rs, src);
        Allocation refRgbAlloc = Allocation.createFromBitmap(rs, ref);

        Allocation srcAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), srcRgbAlloc.getType().getX(), srcRgbAlloc.getType().getY()));
        Allocation refAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), refRgbAlloc.getType().getX(), refRgbAlloc.getType().getY()));

        Allocation srcIdxAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), srcRgbAlloc.getType().getX() >> 2, srcRgbAlloc.getType().getY() >> 2));
        Allocation refIdxAlloc = Allocation.createTyped(rs, Type.createXY(rs, Element.U8(rs), refRgbAlloc.getType().getX() >> 2, refRgbAlloc.getType().getY() >> 2));


        script.forEach_toGrey(srcRgbAlloc, srcAlloc);
        script.forEach_toGrey(refRgbAlloc, refAlloc);

        script.set_BuildIdxRef(srcAlloc);
        script.forEach_buildIdx(srcIdxAlloc, srcIdxAlloc);
        script.set_BuildIdxRef(refAlloc);
        script.forEach_buildIdx(refIdxAlloc, refIdxAlloc);

        int mbWidth  = (src.getWidth() + 15) >> 4;
        int mbHeight = (src.getHeight() + 15) >> 4;
        for (int mbY = 0; mbY < mbHeight; mbY++) {
            for (int mbX = 0; mbX < mbWidth; mbX++) {
                script.invoke_calcMotion(srcIdxAlloc, refIdxAlloc, srcAlloc, refAlloc, mbX, mbY);
            }
        }
        rs.finish();
    }
}
