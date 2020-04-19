import React from "react";
import { useContext } from "react";
import { MapBoxContext } from "./MapBoxContext";
import { useRef, useLayoutEffect, useState } from "react";
import ReactMapGL from "react-map-gl";
import { CanvasOverlay } from "react-map-gl";
import { LngLatBounds } from "mapbox-gl";
import { OpenLocationCode } from "open-location-code";
import * as turf from "@turf/turf";

function BoundingBoxOverlay({ boundingBox, color }) {
  function redraw({ width, height, ctx, isDragging, project, unproject }) {
    const center = project(boundingBox.getCenter().toArray());
    ctx.clearRect(0, 0, width, height);
    ctx.beginPath();
    ctx.arc(center[0], center[1], 10.0, 0, 2 * Math.PI, false);
    ctx.fillStyle = "green";
    ctx.fill();

    const topLeft = project(boundingBox.getNorthWest().toArray());
    const bottomRight = project(boundingBox.getSouthEast().toArray());
    ctx.beginPath();
    ctx.rect(
      topLeft[0],
      topLeft[1],
      bottomRight[0] - topLeft[0],
      bottomRight[1] - topLeft[1]
    );
    ctx.lineWidth = 3;
    ctx.strokeStyle = color;
    ctx.stroke();
  }

  return <CanvasOverlay redraw={redraw} />;
}

function reticuleFromMapBounds(bounds) {
  const northSouthExtent = bounds.getSouth() - bounds.getNorth();
  const westEastExtent = bounds.getEast() - bounds.getWest();

  const indent = 0.2;

  const reticuleBounds = LngLatBounds.convert([
    [
      bounds.getWest() + indent * westEastExtent,
      bounds.getNorth() + indent * northSouthExtent,
    ],
    [
      bounds.getWest() + (1.0 - indent) * westEastExtent,
      bounds.getNorth() + (1.0 - indent) * northSouthExtent,
    ],
  ]);

  return reticuleBounds;
}

function olcReticuleFromMap(map) {
  const center = map.getCenter();
  console.dir(center);
  const [lng, lat] = center.toArray();
  const olc_code = new OpenLocationCode().encode(lat, lng);
  console.dir(olc_code);

  const point = turf.point([lng, lat]);
  const radius = 5;
  const buffered = turf.buffer(point, radius, { units: "kilometers" });
  console.dir(buffered);
  const [minX, minY, maxX, maxY] = turf.bbox(buffered);

  const reticuleBounds = LngLatBounds.convert([
    [maxX, maxY],
    [minX, minY],
  ]);

  console.dir(reticuleBounds);

  return reticuleBounds;
}

export function MapView({ city }) {
  const mapbox = useContext(MapBoxContext);
  const containerRef = useRef(null);
  const [containerDimensions, setContainerDimensions] = useState({
    width: 400,
    height: 800,
  });
  const [reticuleBounds, setReticuleBounds] = useState(null);
  const [fixedReticuleBounds, setFixedReticuleBounds] = useState(null);

  useLayoutEffect(() => {
    const { width, height } = containerRef.current.getBoundingClientRect();
    setContainerDimensions({ width, height });
  }, [containerRef]);

  const { width, height } = containerDimensions;

  const [viewport, setViewport] = useState({
    zoom: mapbox.default_zoom,
    ...city.location,
  });
  function viewportUpdated(viewport) {
    const { zoom, latitude, longitude } = viewport;
    setViewport({ zoom, latitude, longitude });
  }

  function onLoad({ target }) {
    const map = target;
    console.log("loaded");
    setReticuleBounds(reticuleFromMapBounds(map.getBounds()));
    setFixedReticuleBounds(olcReticuleFromMap(map));
    map.on("moveend", () => {
      setReticuleBounds(reticuleFromMapBounds(map.getBounds()));
      setFixedReticuleBounds(olcReticuleFromMap(map));
    });
  }

  return (
    <div ref={containerRef} className="map">
      <ReactMapGL
        width={width}
        height={height}
        {...viewport}
        onViewportChange={viewportUpdated}
        mapboxApiAccessToken={mapbox.access_token}
        onLoad={onLoad}
      >
        {/* {reticuleBounds && (
          <BoundingBoxOverlay boundingBox={reticuleBounds} color="red" />
        )} */}
        {fixedReticuleBounds && (
          <BoundingBoxOverlay boundingBox={fixedReticuleBounds} color="green" />
        )}
      </ReactMapGL>
    </div>
  );
}
