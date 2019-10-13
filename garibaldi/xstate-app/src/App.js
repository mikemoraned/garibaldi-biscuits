import React from "react";
import "./App.css";
import { Machine } from "xstate";
import { useMachine } from "@xstate/react";

const fetchMap = () => {
  console.log("fetching map...");
  return new Promise((resolve, reject) => {
    setTimeout(function() {
      const random = Math.random();
      console.log(random);
      if (random > 0.5) {
        resolve();
      } else {
        reject();
      }
    }, 1000);
  });
};

const mapMachine = Machine({
  id: "map",
  initial: "loading",
  states: {
    loading: {
      invoke: {
        id: "fetchMap",
        src: (context, event) => fetchMap(),
        onDone: {
          target: "interactive"
        },
        onError: {
          target: "loading_failed"
        }
      }
    },
    loading_failed: {
      on: {
        RETRY: "loading"
      }
    },
    interactive: {
      on: {
        RETRY: "loading"
      }
    }
  }
});

function Reload() {
  const [current, send] = useMachine(mapMachine);

  return <button onClick={() => send("RETRY")}>Retry</button>;
}

function Map() {
  const [current, send] = useMachine(mapMachine);

  if (current.matches("loading")) {
    return <div>Loading</div>;
  } else if (current.matches("loading_failed")) {
    return (
      <div>
        <div>Loading failed</div>
        <Reload />
      </div>
    );
  } else if (current.matches("interactive")) {
    return (
      <div>
        <div>Interactive</div>
        <Reload />
      </div>
    );
  } else {
    return <div></div>;
  }
}

function App() {
  return (
    <div className="App">
      <Map />
    </div>
  );
}

export default App;
