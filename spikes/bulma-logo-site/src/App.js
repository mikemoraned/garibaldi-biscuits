import React from "react";
import "./App.scss";

function App() {
  return (
    <div classNameName="App">
      <nav className="navbar" role="navigation" aria-label="main navigation">
        <div className="navbar-brand">
          <a className="navbar-item" href="https://garibaldi.houseofmoran.io/">
            <img src="garibaldi_100w.png" alt="logo" />
            <span
              style={{
                "margin-left": "0.5em",
                font: "small-caps bold 25px monospace",
              }}
            >
              GARIBALDI
            </span>
          </a>
        </div>
      </nav>

      <section className="section">
        <div className="container">
          <h1 className="title">Hello World</h1>
          <p className="subtitle">
            My first website with <strong>Bulma</strong>!
          </p>
        </div>
      </section>
    </div>
  );
}

export default App;
