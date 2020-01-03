import React from "react";
import { CanvasOverlay } from "react-map-gl";
import { interpolateRainbow } from "d3-scale-chromatic";
import { geoJsonBoundsFromLngLatBounds } from "./overlayHelpers";
import { createFeatureRenderer } from "./featureRenderer";

export function lazyLoader() {
  console.time("loading biscuiting libs");
  return Promise.all([
    import("@mike_moran/biscuiting-lib"),
    import("@mike_moran/biscuiting-lib/biscuiting_lib_bg")
  ]).then(([biscuiting_lib, biscuiting_lib_bg]) => {
    console.timeEnd("loading biscuiting libs");
    return bindBiscuitsOverlay({
      biscuiting_lib,
      biscuiting_lib_bg
    });
  });
}

function bindBiscuitsOverlay({ biscuiting_lib, biscuiting_lib_bg }) {
  const { BiscuitFinder } = biscuiting_lib;
  const { memory } = biscuiting_lib_bg;

  const BiscuitsOverlay = ({ boundingBox, featureLoader }) => {
    function redraw({ width, height, ctx, isDragging, project, unproject }) {
      ctx.clearRect(0, 0, width, height);

      const geoJsonBounds = geoJsonBoundsFromLngLatBounds(boundingBox);

      const featureRenderer = createFeatureRenderer(project, ctx);

      if (!isDragging) {
        console.time("- redraw: biscuits");
        const biscuitFinder = BiscuitFinder.new();

        // FIXME: logically, I want to get top left / bottom right of bounding box
        // by asking for north west / south east points. however, that doesn't seem to
        // work and instead I need to get south west / north east instead.
        // const boundingBoxTopLeft = project(boundingBox.getNorthWest().toArray());
        // const boundingBoxBottomRight = project(
        //   boundingBox.getSouthEast().toArray()
        // );
        const boundingBoxTopLeft = project(
          boundingBox.getSouthWest().toArray()
        );
        const boundingBoxBottomRight = project(
          boundingBox.getNorthEast().toArray()
        );

        const boundingBoxMinX = Math.floor(boundingBoxTopLeft[0]);
        const boundingBoxMinY = Math.floor(boundingBoxTopLeft[1]);
        const boundingBoxMaxX = Math.ceil(boundingBoxBottomRight[0]);
        const boundingBoxMaxY = Math.ceil(boundingBoxBottomRight[1]);
        const boundingBoxWidth = boundingBoxMaxX - boundingBoxMinX;
        const boundingBoxHeight = boundingBoxMaxY - boundingBoxMinY;

        console.time("-- drawing map");
        ctx.fillStyle = "black";
        ctx.fillRect(
          boundingBoxMinX,
          boundingBoxMinY,
          boundingBoxWidth,
          boundingBoxHeight
        );

        const features = featureLoader();
        const geoJson = { type: "FeatureCollection", features };

        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.strokeStyle = "white";
        featureRenderer(geoJson);
        ctx.stroke();

        console.timeEnd("-- drawing map");

        console.time("-- getImageData");
        const inputImageData = ctx.getImageData(
          boundingBoxMinX * window.devicePixelRatio,
          boundingBoxMinY * window.devicePixelRatio,
          boundingBoxWidth * window.devicePixelRatio,
          boundingBoxHeight * window.devicePixelRatio
        );
        console.timeEnd("-- getImageData");

        console.time("-- find_biscuits");
        biscuitFinder.find_biscuits(
          boundingBoxWidth * window.devicePixelRatio,
          boundingBoxHeight * window.devicePixelRatio,
          inputImageData.data
        );
        console.timeEnd("-- find_biscuits");

        console.time("-- get borders back");
        const numBorders = biscuitFinder.num_borders();
        const borderIndexesPointer = biscuitFinder.border_indexes_ptr();
        const borderIndexes = new Uint32Array(
          memory.buffer,
          borderIndexesPointer,
          numBorders
        );
        const numBorderPoints = biscuitFinder.num_border_points();
        const borderPointsPointer = biscuitFinder.border_points_ptr();
        const borderPoints = new Uint32Array(
          memory.buffer,
          borderPointsPointer,
          2 * numBorderPoints
        );
        console.timeEnd("-- get borders back");

        console.time("-- draw biscuits");
        ctx.beginPath();
        ctx.fillStyle = "black";
        ctx.rect(
          boundingBoxMinX,
          boundingBoxMinY,
          boundingBoxWidth,
          boundingBoxHeight
        );
        ctx.fill();

        const sampleEvery = 1;
        for (let borderNum = 0; borderNum < numBorders; borderNum++) {
          if (borderNum % sampleEvery === 0) {
            let borderPointStartIndex =
              borderNum === 0 ? 0 : borderIndexes[borderNum - 1];
            let borderPountEndIndex = borderIndexes[borderNum];

            let borderPointIndex = borderPointStartIndex;
            let x =
              boundingBoxMinX +
              borderPoints[borderPointIndex] / window.devicePixelRatio;
            let y =
              boundingBoxMinY +
              borderPoints[borderPointIndex + 1] / window.devicePixelRatio;
            ctx.moveTo(x, y);
            borderPointIndex += 2;

            ctx.beginPath();
            const t = Math.random();
            ctx.fillStyle = interpolateRainbow(t);
            while (borderPointIndex < borderPountEndIndex) {
              const x =
                boundingBoxMinX +
                borderPoints[borderPointIndex] / window.devicePixelRatio;
              const y =
                boundingBoxMinY +
                borderPoints[borderPointIndex + 1] / window.devicePixelRatio;
              ctx.lineTo(x, y);
              borderPointIndex += 2;
            }
            ctx.fill();

            borderPointStartIndex = borderPountEndIndex;
          }
        }
        console.timeEnd("-- draw biscuits");

        console.timeEnd("- redraw: biscuits");
      }

      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.setLineDash([15, 15]);
      ctx.strokeStyle = "green";
      featureRenderer(geoJsonBounds);
      ctx.stroke();
    }

    return <CanvasOverlay redraw={redraw} />;
  };
  return { default: BiscuitsOverlay };
}
