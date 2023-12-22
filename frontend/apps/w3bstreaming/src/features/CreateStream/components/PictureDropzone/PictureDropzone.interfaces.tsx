import { DropzoneUploaderProps } from '@/ui/DropzoneUploader/DropzoneUploader.interface';

export interface PictureDropzoneProps extends DropzoneUploaderProps {
  uploadConfig: {
    address: string;
    gateway: string;
  };
}
