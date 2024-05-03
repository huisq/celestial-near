"use client";
import axios from "axios";
import React, { useEffect, useState } from "react";
import Link from "next/link";
import * as nearAPI from "near-api-js";
import { useRouter } from 'next/navigation';

const Navbar = () => {

  const router = useRouter();

  const [avatarUrl, setAvatarUrl] = useState("");
  const [nearConnection, setNearConnection] = useState();
  const [wallet, setWallet] = useState();

  useEffect(() => {
    const fetchData = async () => {
      try {
        const getRandomNumber = () => Math.floor(Math.random() * 1000);
        const apiUrl = `https://api.multiavatar.com/${getRandomNumber()}`;

        const response = await axios.get(apiUrl);
        const svgDataUri = `data:image/svg+xml,${encodeURIComponent(response.data)}`;
        setAvatarUrl(svgDataUri);
      } catch (error) {
        console.error('Error fetching avatar:', error.message);
      }
    };

    fetchData();
  }, []);

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

  // Creating a new wallet instance

  // // connect to NEAR
  // // const nearConnection = connect(connectionConfig);
  // Creating a near connection

    connect(connectionConfig).then((result) => {
      console.log(result);
      setWallet(new WalletConnection(result, 'testnet'))
      setNearConnection(result);
    }).then(()=>{console.log(nearConnection);});
  }
  }, [])


  function log() {
    // setWallet(new WalletConnection(nearConnection, "testnet"));
    // const wallet = new WalletConnection(nearConnection, "testnet");
    wallet.requestSignIn({});
  }

  function logout() {
    // setWallet(new WalletConnection(nearConnection, "testnet"));
    wallet.signOut();
    console.log(wallet.isSignedIn());
    router.refresh();
  }

  return (
    <div>
      <div className="flex gap-4">
          {/* <Link href="/profile"> */}
            {avatarUrl && <img src={avatarUrl} alt="Avatar" style={{width: 45}}/>} 
            {/* </Link> */}
          {wallet && wallet.isSignedIn() ? (<></>): (
            <button onClick={log} className="bg-white text-black" style={{borderRadius:'30px', paddingLeft:'20px', paddingRight:'20px'}}>
            Connect to Near
            </button>
            )}
          {wallet && wallet.isSignedIn() && (<div className="flex gap-4">
        <p onClick={() => console.log(wallet.account())} className="text-lg text-white bg-black rounded-lg p-2" style={{paddingTop:'10px'}}>{wallet.getAccountId()}</p>
      <button
        onClick={logout}
        className="align-center bg-blue-500 hover:bg-blue-700 text-white font-bold px-2 rounded-lg focus:outline-none focus:shadow-outline"
      >
        Log Out
      </button>
    </div>)}
          </div>
    </div>
  );
};

export default Navbar;
