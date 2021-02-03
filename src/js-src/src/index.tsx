import { FC, useState } from "react";
import * as React from "react";
import * as ReactDOM from "react-dom";

const App: FC = () => {
  const [count, setCount] = useState(0);

  return (
    <div>
      <h1>Hello, World</h1>
      <div>
        <p>{count}</p>
        <button onClick={() => setCount((p) => p - 1)}>-</button>
        <button onClick={() => setCount((p) => p + 1)}>+</button>
      </div>
    </div>
  );
};

ReactDOM.render(
  <>
    <App />
  </>,
  document.getElementById("root")
);
