import { DropzoneUploader } from '@ui';
import { ADDRESS } from '@/consts';

function PictureDropzone({ ...props }: any) {
  return (
    <DropzoneUploader
      uploadConfig={{
        address: ADDRESS.IPFS_NODE,
        gateway: ADDRESS.IPFS_GATEWAY,
      }}
      {...props}
    />
  );
}

export { PictureDropzone };
