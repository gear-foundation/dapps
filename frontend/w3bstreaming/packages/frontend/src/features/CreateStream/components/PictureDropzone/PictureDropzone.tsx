import { DropzoneUploader } from '@ui';

function PictureDropzone({ text, onDropFile }: any) {
  return <DropzoneUploader onDropFile={onDropFile} text={text} />;
}

export { PictureDropzone };
