import Head from "next/head";
import { createGlobalStyle } from "styled-components";

const GlobalStyle = createGlobalStyle`
body {
  margin: 0;
  overflow: hidden;
  background-color: #000;
}
`;

const MyApp = ({ Component, pageProps }) => {
  return (
    <>
      <GlobalStyle />
      <Head>
        <meta charSet="utf-8" />
        <meta
          name="viewport"
          content="minimum-scale=1, initial-scale=1, width=device-width, shrink-to-fit=no"
        />
      </Head>
      <Component {...pageProps} />
    </>
  );
};

export default MyApp;
