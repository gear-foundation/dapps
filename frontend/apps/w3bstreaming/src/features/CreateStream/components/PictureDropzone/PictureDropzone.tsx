import { ADDRESS } from '@/consts';
import { DropzoneUploader } from '@/ui';

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
