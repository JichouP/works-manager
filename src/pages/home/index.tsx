import { VFC } from 'react';
import { convertFileSrc } from '@tauri-apps/api/tauri';

const assetUrl = convertFileSrc('D:/ピクチャ/38.png');

const Home: VFC = () => {
  return (
    <div>
      <p>{assetUrl}</p>
      <img src={assetUrl}></img>
    </div>
  );
};

export default Home;
