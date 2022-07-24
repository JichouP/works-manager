import DotsIcon from '@/icons/DotsIcon';
import MenuIcon from '@/icons/MenuIcon';
import { VFC } from 'react';

type Props = {
  title: string;
};

const Navbar: VFC<Props> = ({ title }) => {
  return (
    <div className='navbar bg-base-300'>
      <div className='flex-none'>
        <label
          htmlFor='drawer-sidebar'
          className='btn btn-square btn-ghost lg:hidden'
        >
          <MenuIcon></MenuIcon>
        </label>
      </div>
      <div className='flex-1'>
        <a className='text-xl normal-case btn btn-ghost btn-disabled'>
          {title}
        </a>
      </div>
      <div className='flex-none'>
        <button className='btn btn-square btn-ghost'>
          <DotsIcon></DotsIcon>
        </button>
      </div>
    </div>
  );
};

export default Navbar;
