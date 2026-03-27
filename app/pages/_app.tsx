import { ChakraProvider } from '@chakra-ui/react';
import { SolanaWalletProvider } from '@solana/wallet-adapter-react';

function MyApp({ Component, pageProps }) {
  return (
    <ChakraProvider>
      <SolanaWalletProvider>
        <Component {...pageProps} />
      </SolanaWalletProvider>
    </ChakraProvider>
  );
}

export default MyApp;