export interface DropzoneUploaderProps {
  onDropFile: (prev: string) => void;
  previewLink?: string;
  text?: string;
}
