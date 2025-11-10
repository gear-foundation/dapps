import { ENV } from '@/consts';
import { DropzoneUploader } from '@/ui';
import { DropzoneUploaderProps } from '@/ui/DropzoneUploader/DropzoneUploader.interface';

type PictureDropzoneProps = Omit<DropzoneUploaderProps, 'uploadConfig'>;

function PictureDropzone(props: PictureDropzoneProps) {
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
