import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

const App = () => {
  const [viewCount, setViewCount] = useState<null | number>(null);
  return (
    <div className="flex flex-col bg-neutral-950 min-h-screen">
      <header className="flex items-center justify-between p-6">
        <div className="flex items-center gap-1">
          <div
            className={`w-6 h-6 bg-indigo-400 [mask-image:url("/assets/icons/arrow-down.svg")] [mask-size:24px]`}
          />
          <h1 className="text-neutral-100 text-xl">Ripper</h1>
        </div>
        <div
          className={`w-6 h-6 bg-neutral-600 [mask-image:url("/assets/icons/settings.svg")] [mask-size:24px] hover:bg-violet-400 cursor-pointer`}
        />
      </header>
      <div className="flex flex-col justify-center items-center">
        <span className="text-neutral-100 text-xl"> {viewCount} </span>
        <button
          className="bg-violet-500 p-2 rounded-md"
          onClick={async () => {
            const views: number = await invoke("get_video", {
              url: "https://www.youtube.com/watch?v=Z5NoQg8LdDk",
            })

            setViewCount(views)

          }
          }
        >
          GET DATA
        </button>
      </div>
    </div>
  );
};

export default App;
