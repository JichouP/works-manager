const Sidebar = () => {
  return (
    <div className='drawer-side'>
      <label htmlFor='drawer-sidebar' className='drawer-overlay'></label>
      <ul className='p-4 overflow-y-auto menu w-80 bg-base-200 text-base-content'>
        <li>
          <a>Sidebar Item 1</a>
        </li>
        <li>
          <a>Sidebar Item 2</a>
        </li>
      </ul>
    </div>
  );
};

export default Sidebar;
