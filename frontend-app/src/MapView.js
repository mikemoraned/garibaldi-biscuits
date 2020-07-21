import React from "react";
import { useContext, Suspense } from "react";
import { MapBoxContext } from "./MapBoxContext";
import { useRef, useLayoutEffect, useState } from "react";
import ReactMapGL from "react-map-gl";
import { LngLatBounds } from "mapbox-gl";
import { lazyLoader } from "./BiscuitsOverlay";
import { FeatureOverlay } from "./FeatureOverlay";

const BiscuitsOverlay = React.lazy(lazyLoader);

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

function ToggleControl({ toggles }) {
  return (
    <>
      <div className="buttons is-hidden-tablet are-small has-addons">
        {toggles.map((t) => {
          return (
            <button
              key={t.name.short}
              className={`button is-info ${t.value ? "" : "is-inverted"}`}
              onClick={() => t.setter(!t.value)}
            >
              {t.name.short}
            </button>
          );
        })}
      </div>
      <div className="buttons is-hidden-mobile has-addons">
        {toggles.map((t) => {
          return (
            <button
              key={t.name.full}
              className={`button is-info ${t.value ? "" : "is-inverted"}`}
              onClick={() => t.setter(!t.value)}
            >
              {t.name.full}
            </button>
          );
        })}
      </div>
    </>
  );
}

export function MapView({ city }) {
  const [showStreets, setShowStreets] = useState(false);
  const [showBackgroundStreets, setShowBackgroundStreets] = useState(true);
  const [showBackgroundWater, setShowBackgroundWater] = useState(true);
  const [showBiscuits, setShowBiscuits] = useState(true);

  const mapbox = useContext(MapBoxContext);
  const containerRef = useRef(null);
  const [containerDimensions, setContainerDimensions] = useState({
    width: 400,
    height: 800,
  });
  const [reticuleBounds, setReticuleBounds] = useState(null);

  const [
    reticuleBoundsWaterFeatureLoader,
    setReticuleBoundsWaterFeatureLoader,
  ] = useState(null);
  const [
    reticuleBoundsFeatureLoader,
    setReticuleBoundsFeatureLoader,
  ] = useState(null);
  const [featureLoader, setFeatureLoader] = useState(null);

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

  function onBoundsChanged(map) {
    const filter = [
      "in",
      "class",
      "street",
      "pedestrian",
      "motorway",
      "motorway_link",
      "path",
      "primary",
      "primary_link",
      "secondary",
      "secondary_link",
      "tertiary",
      "tertiary_link",
      "track",
    ];
    const reticule = reticuleFromMapBounds(map.getBounds());
    setFeatureLoader(() => {
      return () => {
        return map.queryRenderedFeatures({
          filter,
        });
      };
    });
    setReticuleBounds(reticule);
    setReticuleBoundsWaterFeatureLoader(() => {
      return () => {
        return map.queryRenderedFeatures(
          [
            map.project(reticule.getNorthEast().toArray()),
            map.project(reticule.getSouthWest().toArray()),
          ],
          {
            layers: ["water"],
          }
        );
      };
    });
    setReticuleBoundsFeatureLoader(() => {
      return () => {
        return map.queryRenderedFeatures(
          [
            map.project(reticule.getNorthEast().toArray()),
            map.project(reticule.getSouthWest().toArray()),
          ],
          {
            filter,
          }
        );
      };
    });
  }

  function onLoad({ target }) {
    const map = target;
    console.log("loaded");
    onBoundsChanged(map);
    map.on("moveend", () => {
      onBoundsChanged(map);
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
        <>
          <Suspense fallback={<div></div>}>
            {reticuleBounds &&
              reticuleBoundsFeatureLoader != null &&
              reticuleBoundsWaterFeatureLoader != null &&
              (showBackgroundStreets ||
                showBackgroundWater ||
                showBiscuits) && (
                <BiscuitsOverlay
                  boundingBox={reticuleBounds}
                  featureLoader={reticuleBoundsFeatureLoader}
                  waterFeatureLoader={reticuleBoundsWaterFeatureLoader}
                  showBackgroundStreets={showBackgroundStreets}
                  showBackgroundWater={showBackgroundWater}
                  showBiscuits={showBiscuits}
                />
              )}
          </Suspense>
          {reticuleBounds && featureLoader != null && showStreets && (
            <FeatureOverlay
              boundingBox={reticuleBounds}
              featureLoader={featureLoader}
            />
          )}
          <div style={{ position: "absolute", top: 0, right: 0 }}>
            <ToggleControl
              toggles={[
                {
                  name: {
                    full: "Streets",
                    short: "S",
                  },

                  value: showStreets,
                  setter: setShowStreets,
                },
                {
                  name: {
                    full: "Bg: Streets",
                    short: "Bg: S",
                  },
                  value: showBackgroundStreets,
                  setter: setShowBackgroundStreets,
                },
                {
                  name: { full: "Bg: Water", short: "Bg: W" },
                  value: showBackgroundWater,
                  setter: setShowBackgroundWater,
                },
                {
                  name: { full: "Biscuits", short: "B" },
                  value: showBiscuits,
                  setter: setShowBiscuits,
                },
              ]}
            />
          </div>
        </>
      </ReactMapGL>
    </div>
  );
}
