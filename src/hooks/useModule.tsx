import { useEffect, useState } from "react";

type ModuleType =
  typeof import("../../mandelbrot-generator/pkg/mandelbrot_generator");

const useModule = () => {
  const [module, setModule] = useState<ModuleType | undefined>();

  useEffect(() => {
    if (process.browser) {
      import("../../mandelbrot-generator/pkg/mandelbrot_generator").then(
        (mod) => setModule(mod)
      );
    }
  });

  return module;
};
export default useModule;
