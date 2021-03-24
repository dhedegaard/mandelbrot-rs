import useModule from "../hooks/useModule";

const Index = () => {
  const module = useModule();
  console.log(module);

  return (
    <>
      <h1>Mandelbrot?</h1>
      <button
        onClick={() => {
          console.log(module?.mandelbrot(800, 600));
        }}
      >
        Click me
      </button>
    </>
  );
};

export default Index;
