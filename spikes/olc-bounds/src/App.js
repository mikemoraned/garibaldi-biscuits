import React from "react";
import "./App.scss";
import { MapView } from "./MapView";
import dotenv from "dotenv";

import { MapBoxContextProviderFromEnvironment } from "./MapBoxContext";

dotenv.config();

const cities = [
  {
    name: "San Francisco",
    location: {
      latitude: 37.774929,
      longitude: -122.419418,
      plus_code: "849VQHFJ+X6",
    },
  },
  {
    name: "Jerusalem",
    location: {
      latitude: 31.768318,
      longitude: 35.213711,
      plus_code: "8G3QQ697+8F",
    },
  },
  {
    name: "Berlin",
    location: {
      latitude: 52.520008,
      longitude: 13.404954,
      plus_code: "9F4MGCC3+2X",
    },
  },
  {
    name: "London",
    location: {
      latitude: 51.507351,
      longitude: -0.127758,
      plus_code: "9C3XGV4C+WV",
    },
  },
];

function App() {
  return (
    <MapBoxContextProviderFromEnvironment>
      <div>
        <MapView city={cities[3]} />
      </div>
    </MapBoxContextProviderFromEnvironment>
  );
}

export default App;
