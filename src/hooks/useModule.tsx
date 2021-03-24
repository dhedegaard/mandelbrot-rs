import { useEffect, useState } from "react";

const modulePromise = import(
  "../../mandelbrot-generator/pkg/mandelbrot_generator"
);
type ModuleType = typeof import("../../mandelbrot-generator/pkg/mandelbrot_generator");

const useModule = () => {
  const [module, setModule] = useState<ModuleType | undefined>();

  useEffect(() => {
    modulePromise.then((mod) => setModule(mod));
  });

  return module;
};
export default useModule;
