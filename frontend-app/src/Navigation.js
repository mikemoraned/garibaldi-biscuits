import React from "react";

export function Navigation() {
  return (
    <nav
      className="navbar is-light"
      role="navigation"
      aria-label="main navigation"
    >
      <div className="navbar-brand">
        <a className="navbar-item" href="https://garibaldi.houseofmoran.io/">
          <img src="garibaldi-2_100w.png" alt="logo" />
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
  );
}
