import { ENV } from '@/consts';
import { DropzoneUploader } from '@/ui';

function PictureDropzone({ ...props }: any) {
  return (
    <DropzoneUploader
      uploadConfig={{
        address: ENV.IPFS_NODE,
        gateway: ENV.IPFS_GATEWAY,
      }}
      {...props}
    />
  );
}

export { PictureDropzone };
