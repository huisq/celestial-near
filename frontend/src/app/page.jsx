"use client";
import Image from "next/image";
import Link from "next/link";
import { useState, useEffect } from "react";
import Navbar from "../../components/Navbar";
import Cookies from "js-cookie";
import axios from "axios";
import * as nearAPI from "near-api-js";
import { useSearchParams } from 'next/navigation';
const { Contract } = nearAPI;
const graphqlaptos = "https://sui-devnet.mystenlabs.com/graphql";

export default function Home() {
  const [drawnCard, setDrawnCard] = useState(null);
  const [loading, setLoading] = useState(false);
  const [ques, setques] = useState(false);
  const [description, setDescription] = useState("");
  const [lyrics, setLyrics] = useState("");
  const [cardimage, setcardimage] = useState("");
  const [position, setposition] = useState("");
  const [mintdone, setmintdone] = useState(false);

  // Creating a new wallet instance
  const [walletnear, setWalletnear] = useState();
  // Creating a near connection
  const [nearConnection, setNearConnection] = useState();

  const handleDrawCardAndFetchreading = async () => {
    setLoading(true);

    try {

      const contract = new Contract(walletnear.account(), "tarotv0.testnet", {
        changeMethods: ["draw_cards"],
      });
      const drawResponse = await contract.draw_cards(
        {
          arg_name: "", // argument name and value - pass empty object if no args required
        },
        "30000000000000", // attached GAS (optional)
        "500000000000000000000000" // attached deposit in yoctoNEAR (optional)
      ).then((drawResponse) => {
        console.log("Drawn Card Transaction:", drawResponse);
        // Process drawResponse here if needed
      });

    } catch (error) {
      console.error("Error handling draw card and fetching reading:", error);
    } finally {
      setLoading(false);
    }
  };

  const mintreading = async () => {
    setLoading(true);

    try {

      const accountId = walletnear?.getAccountId();

      const contract = new Contract(walletnear.account(), "tarotv0.testnet", {
        changeMethods: ["nft_mint"],
      });

      const mintRes = await contract.nft_mint(
        // {
        //   callbackUrl: "http://localhost:3000/", // callbackUrl after the transaction approved (optional)
        //   meta: "mintsuccess", // meta information NEAR Wallet will send back to the application. `meta` will be attached to the `callbackUrl` as a url param
        //   args: {
        //     receiver_id: accountId,
        //     question: description, 
        //     reading: lyrics,
        //     card: drawnCard,
        //     position: position// argument name and value - pass empty object if no args required
        //   },
        //   gas: 30000000000000, // attached GAS (optional)
        //   amount: 1000000000000000000000000, // attached deposit in yoctoNEAR (optional)
        // }
        {
          receiver_id: accountId,
          question: description, 
          reading: lyrics,
          card: drawnCard,
          position: position// argument name and value - pass empty object if no args required
        },
        "30000000000000", // attached GAS (optional)
        "1060000000000000000000000" // attached deposit in yoctoNEAR (optional)
      ).then((mintResponse) => {
        console.log("Drawn Card Transaction:", mintResponse);
        // Process drawResponse here if needed
      });

      setmintdone(true);
    } catch (error) {
      console.error("Error handling draw card and fetching rap lyrics:", error);
    } finally {
      setLoading(false);
    }
  };

  //------------------------------------------- check NEAR wallet connection -------------------------------------------------------
  
  useEffect(()=>{
    if (typeof window !== 'undefined') {
  const { connect, keyStores, WalletConnection } = nearAPI;

  // creates keyStore using private key in local storage
  const myKeyStore = new keyStores.BrowserLocalStorageKeyStore();

  // Connecting to NEAR
  const connectionConfig = {
    networkId: "testnet",
    keyStore: myKeyStore, // first create a key store
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://testnet.mynearwallet.com/",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://testnet.nearblocks.io",
  };

    connect(connectionConfig).then((result) => {
      console.log(result);
      setWalletnear(new WalletConnection(result, 'testnet'))
      setNearConnection(result);
    }).then(()=>{console.log(nearConnection);});
  }
  }, [])


  // ----------------------------------------------------------- draw card transaction output --------------------------------------------------------------

  async function initNear() {
    if (typeof window !== 'undefined') {
    const { connect, keyStores, WalletConnection } = nearAPI;

    const myKeyStore = new keyStores.BrowserLocalStorageKeyStore();

    const connectionConfig = {
      networkId: "testnet",
      keyStore: myKeyStore, // first create a key store
      nodeUrl: "https://rpc.testnet.near.org",
      walletUrl: "https://testnet.mynearwallet.com/",
      helperUrl: "https://helper.testnet.near.org",
      explorerUrl: "https://testnet.nearblocks.io",
    };

    const near = await connect(connectionConfig);
    const wallet = new WalletConnection(near);
    return { near, wallet };
  }
}

async function getTransactionResult(txHash, accountId) {
  const { near } = await initNear();
  const provider = near.connection.provider;

  try {
      const txResult = await provider.txStatus(txHash, accountId);

      console.log("txresult", txResult);

      if(txResult.transaction.actions[0].FunctionCall.method_name == "nft_mint")
      {
        setmintdone(true);
      }

      const outcome = txResult.transaction_outcome.outcome;
      const logs = outcome.logs;
      const successValue = txResult.status.SuccessValue;

      if (successValue) {
          const decodedOutput = atob(successValue);
          console.log('Decoded Output:', decodedOutput);
          return decodedOutput;
      } else {
          console.log('Transaction logs:', logs);
          return logs;
      }
  } catch (error) {
      console.error('Error fetching transaction:', error);
      throw error;
  }
}

// Get the URL fragment
const searchParams = useSearchParams();
const txHash = searchParams.get('transactionHashes')

// AexLFe9xBrXgEdfediyZU7cNzx6e5RQsxyGQFkkc8Js5

const accountId = walletnear?.getAccountId();
useEffect(() => {
  const handleCheck = async () => {
    try {
      setLoading(true);

        const outputString = await getTransactionResult(txHash, `${accountId}`);
        const output = JSON.parse(outputString);
        console.log('Transaction Output:', output);

        const card = output[0];
      const position = output[2];

      console.log("card", output[0], "position", output[2], "cardimg" , output[1]);

      setcardimage(output[1]);
      setDrawnCard(output[0]);
      setposition(output[2]);

      if(card)
      {
        setques(true);
      }

      const requestBody = {
        model: "gpt-4",
        messages: [
          {
            role: "user",
            content: `You are a Major Arcana Tarot reader. Client asks this question “${description}” and draws the “${card}” card in “${position}” position. Interpret to the client in no more than 150 words.`,
          },
        ],
      };
      
      let apiKey = process.env.NEXT_PUBLIC_API_KEY;
      const baseURL = "https://api.openai.com/v1/chat/completions";
      const headers = new Headers();
      headers.append("Content-Type", "application/json");
      headers.append("Accept", "application/json");
      headers.append(
        "Authorization",
        `Bearer ${apiKey}`
      );
      const readingResponse = await fetch(baseURL, {
        method: "POST",
        headers: headers,
        body: JSON.stringify(requestBody),
      });
  

      if (!readingResponse.ok) {
        throw new Error("Failed to fetch reading");
      }

      const readingData = await readingResponse.json();
      setLyrics(readingData.choices[0].message.content);
      console.log(readingData);
      console.log("Data to send in mint:", card, position);
      setLoading(false);
    } catch (error) {
        console.error('Failed to fetch transaction output:', error);
        setLoading(false);
    }
};
if(txHash)
{
  handleCheck();
}
}, [])

  return (
    <main
      className="flex min-h-screen flex-col items-center justify-between lg:p-24 p-10"
      style={{
        backgroundImage: "url(/tarot_design_dark.png)", // Path to your background image
        backgroundSize: "cover", // Adjust as needed
        backgroundPosition: "center", // Adjust as needed
      }}
    >
      <div className="z-10 lg:max-w-6xl w-full justify-between font-mono text-sm lg:flex md:flex">
        <p
          className="text-white text-xl pb-6 backdrop-blur-2xl dark:border-neutral-800 dark:from-inherit rounded-xl p-4"
          style={{
            backgroundColor: "#1F2544",
            boxShadow: "inset -10px -10px 60px 0 rgba(255, 255, 255, 0.4)",
          }}
        >
          Tarot Reading
        </p>
        <div
        >
          <Navbar />
        </div>
      </div>

      <div className="lg:flex md:flex gap-10">
        <div>
          {!ques && (
            <button
              onClick={() => {
                setques(true);
              }}
              className="bg-white rounded-lg py-2 px-8 text-black mt-40 font-bold"
            >
              Ask question
            </button>
          )}

          {ques && walletnear.isSignedIn() && (
            <div
              className="px-10 py-10 bgcolor rounded-2xl mt-10 max-w-xl"
              style={{
                border: "1px solid #0162FF",
                boxShadow: "inset -10px -10px 60px 0 rgba(255, 255, 255, 0.4)",
              }}
            >
              {!lyrics && (
                <>
                  <input
                    type="text"
                    placeholder="Write your question here"
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                    className="p-2 rounded-lg w-full focus:outline-none text-black"
                  />
                  <button
                    onClick={handleDrawCardAndFetchreading}
                    className="mt-20 bg-black rounded-lg py-2 px-8 text-white"
                  >
                    Get my reading
                  </button>

                  <div className="mt-2 text-white">Cost: 0.5 Near</div>
                </>
              )}
              <div>
                {lyrics && (
                  <div>
                    <div className="flex gap-4 pb-8">
                      <button
                        onClick={() => {
                          setques(true);
                          setDrawnCard(null);
                          setLyrics("");
                        }}
                        className="bg-black rounded-lg py-2 px-8 text-yellow-200"
                      >
                        Start Again
                      </button>

                      <button
                        onClick={mintreading}
                        className="bg-yellow-100 rounded-lg py-2 px-6 text-black font-semibold"
                      >
                        Mint reading
                      </button>
                      <div className="mt-2 text-white">Cost: 1 Near</div>

                    </div>
                    <h2 className="font-bold mb-2 text-white">
                      Your Tarot Reading:
                    </h2>
                    <p className="text-white">{lyrics}</p>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>

        {drawnCard && lyrics ? (
          <div>
            <h2 className="mt-10 mb-4 ml-20 text-white">{drawnCard}</h2>
            {position === "upright" ? (
              <img
                src={`${"https://nftstorage.link/ipfs"}/${
                  cardimage.split("ipfs://")[1]
                }`}
                width="350"
                height="350"
              />
            ) : (
              <img
                src={`${"https://nftstorage.link/ipfs"}/${
                  cardimage.split("ipfs://")[1]
                }`}
                width="350"
                height="350"
                style={{ transform: "rotate(180deg)" }}
              />
            )}
          </div>
        ) : (
          <div className="rounded-lg mt-10">
            <img src="/tarot_card.jpg" className="w-full"/>
          </div>
        )}
      </div>

      {ques && !walletnear.isSignedIn() && (
        <div
          style={{ backgroundColor: "#222944E5" }}
          className="flex overflow-y-auto overflow-x-hidden fixed inset-0 z-50 justify-center items-center w-full max-h-full"
          id="popupmodal"
        >
          <div className="relative p-4 lg:w-1/3 w-full max-w-2xl max-h-full">
            <div className="relative rounded-lg shadow bg-black text-white">
              <div className="flex items-center justify-end p-4 md:p-5 rounded-t dark:border-gray-600">
                <button
                  onClick={() => setques(false)}
                  type="button"
                  className="text-white bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white"
                >
                  <svg
                    className="w-3 h-3"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 14 14"
                  >
                    <path
                      stroke="currentColor"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
                    />
                  </svg>
                  <span className="sr-only">Close modal</span>
                </button>
              </div>

              <div className="p-4 space-y-4">
                <p className="text-2xl text-center font-bold" style={{color:'#FFB000'}}>
                Please connect your Near Wallet
                </p>
              </div>
              <div className="flex items-center p-4 rounded-b pb-20 pt-10 justify-center">
                  <Navbar />
              </div>
            </div>
          </div>
        </div>
      )}

      {mintdone && (
        <div
          style={{ backgroundColor: "#222944E5" }}
          className="flex overflow-y-auto overflow-x-hidden fixed inset-0 z-50 justify-center items-center w-full max-h-full"
          id="popupmodal"
        >
          <div className="relative p-4 lg:w-1/3 w-full max-w-2xl max-h-full">
            <div className="relative rounded-lg shadow bg-black text-white">
              <div className="flex items-center justify-end p-4 md:p-5 rounded-t dark:border-gray-600">
                <button
                  onClick={() => setmintdone(false)}
                  type="button"
                  className="text-white bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white"
                >
                  <svg
                    className="w-3 h-3"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 14 14"
                  >
                    <path
                      stroke="currentColor"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
                    />
                  </svg>
                  <span className="sr-only">Close modal</span>
                </button>
              </div>

              <div className="p-4 space-y-4 pb-20">
                <p className="text-3xl text-center font-bold text-green-500">
                  Successfully Minted!!
                </p>
                <p className="text-lg text-center pt-4">
                Please check the NFT in your wallet.
                </p> 
              </div>
              {/* <div className="flex items-center p-4 rounded-b pb-20">
                <Link href="/profile"
                  type="button"
                  className="w-1/2 mx-auto text-black bg-white font-bold focus:ring-4 focus:outline-none focus:ring-blue-300 rounded-lg text-md px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >
                  My Profile
                </Link>
              </div> */}
            </div>
          </div>
        </div>
      )}

      {loading && (
        <div
          style={{ backgroundColor: "#222944E5" }}
          className="flex overflow-y-auto overflow-x-hidden fixed inset-0 z-50 justify-center items-center w-full max-h-full"
          id="popupmodal"
        >
          <div className="relative p-4 lg:w-1/5 w-full max-w-2xl max-h-full">
            <div className="relative rounded-lg shadow">
              <div className="flex justify-center gap-4">
                <img
                  className="w-50 h-40"
                  src="/loader.gif"
                  alt="Loading icon"
                />

                {/* <span className="text-white mt-2">Loading...</span> */}
              </div>
            </div>
          </div>
        </div>
      )}

    </main>
  );
}
