import { useCallback, useState } from "react";
import useModule from "../hooks/useModule";

const Index = () => {
  const module = useModule();
  console.log(module);

  const [imgSrc, setImgSrc] = useState<string | undefined>();

  const handleClick = useCallback(() => {
    const bytes = module.mandelbrot(800, 600, 1000);
    const blob = new Blob([bytes], { type: "image/png" });
    setImgSrc(URL.createObjectURL(blob));
  }, [setImgSrc, module]);

  return (
    <>
      <h1>Mandelbrot?</h1>
      <button onClick={handleClick}>Click me</button>
      {imgSrc != null && (
        <>
          <br />
          <img src={imgSrc} width={800} height={600} alt="mandelbrot" />
        </>
      )}
    </>
  );
};

export default Index;
