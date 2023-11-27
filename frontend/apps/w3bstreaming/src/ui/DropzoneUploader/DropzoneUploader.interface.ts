export interface DropzoneUploaderProps {
  onDropFile: (prev: string[]) => void;
  previewLinks?: string[];
  content?: JSX.Element;
  className?: string;
  multi?: boolean;
  uploadConfig: {
    address: string;
    gateway: string;
  };
}
