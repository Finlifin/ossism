import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Latex from "react-latex-next";

function App() {
  const [output, setOutput] = useState("");
  const [input, setInput] = useState("");

  async function handleSubmit(e: any) {
    e.preventDefault();
    setOutput(await invoke("greet", { input }));
  }

  async function submit() {
    setOutput(output + '<br><br>' + input + ': <br>' + await invoke("request_gpt", { question: input, sysRole: "你是数学助手" }));
    setInput("")
  }

  return (
    <div className="main">
      <div className="output"><Latex>{output}</Latex></div>
      <input type="text" value={input} className="input"
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={(e) => {
          if (e.keyCode == 13) {
            submit()
          }
        }}
        placeholder="Asking something..."
      />
    </div>
  );
}

export default App;
