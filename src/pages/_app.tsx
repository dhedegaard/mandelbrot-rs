import { AppProps } from "next/dist/shared/lib/router/router";
import Head from "next/head";
import { Global } from "@emotion/react";

const MyApp = ({ Component, pageProps }: AppProps) => {
  return (
    <>
      <Global
        styles={`
          body {
            margin: 0;
            overflow: hidden;
            background-color: #000;
          }
        `}
      />
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
