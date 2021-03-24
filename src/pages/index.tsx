import useModule from "../hooks/useModule";

const Index = () => {
  const module = useModule();
  console.log(module);

  return (
    <>
      <h1>Mandelbrot?</h1>
      <button
        onClick={() => {
          const before = new Date().getTime();
          console.log("res:", module?.mandelbrot(800, 600, 10));
          console.log("duration:", new Date().getTime() - before);
        }}
      >
        Click me
      </button>
    </>
  );
};

export default Index;
