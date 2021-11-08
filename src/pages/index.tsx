import { useCallback, useEffect, useRef, useState } from "react";
import styled from "styled-components";
import useModule from "../hooks/useModule";

const Img = styled.img`
  object-fit: fill;
  image-rendering: optimizeQuality;
`;

const Info = styled.div`
  position: absolute;
  top: 0;
  left: 0;
  background-color: rgba(255, 255, 255, 0.5);
  border: 1px solid #fff;
  padding: 2px;
  font-size: small;
  border-radius: 3px;
  color: #fff;
`;

const Index = () => {
  const module = useModule();
  const loadingRef = useRef(false);
  const imageObjRef = useRef<string>();
  const [generationDuration, setGenerationDuration] = useState(0);
  const [iterations, setIterations] = useState(50);

  const [[width, height], setDimensions] = useState([
    typeof window === "undefined" ? 0 : window.innerWidth,
    typeof window === "undefined" ? 0 : window.innerHeight,
  ]);
  useEffect(() => {
    const handler = () => {
      setDimensions([window.innerWidth, window.innerHeight]);
    };
    window.addEventListener("resize", handler, { passive: true });
    return () => window.removeEventListener("resize", handler);
  }, [setDimensions]);

  const [imgSrc, setImgSrc] = useState<string | undefined>();
  const generateImage = useCallback(() => {
    document.body.style.cursor = "wait";
    if (loadingRef.current || width === 0 || height === 0) {
      return;
    }
    if (module == null) {
      console.info("module is not defined");
      return;
    }
    loadingRef.current = true;
    const before = performance.now();
    const bytes = module.mandelbrot(width, height, iterations);
    const blob = new Blob([bytes], { type: "image/bmp" });
    const imageObj = URL.createObjectURL(blob);
    setImgSrc(imageObj);
    if (imageObjRef.current != null) {
      URL.revokeObjectURL(imageObjRef.current);
    }
    imageObjRef.current = imageObj;
    setGenerationDuration(performance.now() - before);
    document.body.style.removeProperty("cursor");
    loadingRef.current = false;
  }, [setImgSrc, module, width, height, iterations]);

  useEffect(() => {
    if (module == null) {
      return;
    }
    generateImage();
  }, [module, width, height, iterations]);

  const decrementIterations = useCallback(
    () => setIterations((old) => Math.max(0, old < 10 ? old - 1 : old - 10)),
    []
  );
  const incrementIterations = useCallback(
    () => setIterations((old) => (old < 10 ? old + 1 : old + 10)),
    []
  );

  if (imgSrc == null) {
    return null;
  }

  return (
    <>
      <Img src={imgSrc} width={width} height={height} alt="mandelbrot" />
      <Info>
        Last render: <b>{generationDuration.toPrecision(6)} ms</b>
        <br />
        Iterations: <b>{iterations}</b>
        <button type="button" onClick={decrementIterations}>
          -
        </button>
        <button type="button" onClick={incrementIterations}>
          +
        </button>
      </Info>
    </>
  );
};

export default Index;
