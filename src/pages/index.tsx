import { useCallback, useEffect, useRef, useState } from "react";
import styled from "styled-components";
import useModule from "../hooks/useModule";

const Img = styled.img`
  pointer-events: none;
  user-select: none;
  object-fit: fill;
  image-rendering: optimizeQuality;
`;

const Index = () => {
  const module = useModule();
  const loadingRef = useRef(false);

  const [[width, height], setDimensions] = useState([
    typeof window === "undefined" ? 0 : window.innerWidth,
    typeof window === "undefined" ? 0 : window.innerHeight,
  ]);
  useEffect(() => {
    const handler = () => {
      setDimensions([window.innerWidth, window.innerHeight]);
    };
    window.addEventListener("resize", handler);
    return () => window.removeEventListener("resize", handler);
  }, [setDimensions]);

  const [imgSrc, setImgSrc] = useState<string | undefined>();
  const generateImage = useCallback(() => {
    document.body.style.cursor = "wait";
    if (loadingRef.current || width === 0 || height === 0) {
      return;
    }
    loadingRef.current = true;
    const bytes = module.mandelbrot(width, height, 100);
    const blob = new Blob([bytes], { type: "image/png" });
    setImgSrc(URL.createObjectURL(blob));
    document.body.style.removeProperty("cursor");
    loadingRef.current = false;
  }, [setImgSrc, module, width, height]);

  useEffect(() => {
    if (module == null) {
      return;
    }
    generateImage();
  }, [module]);

  if (imgSrc == null) {
    return null;
  }

  return (
    <Img
      src={imgSrc}
      width={width}
      height={height}
      onDragStart={() => false}
      alt="mandelbrot"
    />
  );
};

export default Index;
