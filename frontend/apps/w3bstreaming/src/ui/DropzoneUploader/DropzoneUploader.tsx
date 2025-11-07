import { MouseEvent, useEffect, useState } from 'react';
import { useDropzone } from 'react-dropzone';

import closeIcon from '@/assets/icons/cross-icon.svg';
import picImage from '@/assets/icons/picture.png';
import { cx } from '@/utils';

import { Button } from '../Button';

import { DropzoneUploaderProps } from './DropzoneUploader.interface';
import styles from './DropzoneUploader.module.scss';

function DropzoneUploader({
  content,
  uploadConfig,
  onDropFile,
  className,
  multi,
  previewLinks,
}: DropzoneUploaderProps) {
  const uploadUrl = `${uploadConfig.address}`;
  const [preview, setPreview] = useState<string[]>(previewLinks || []);

  useEffect(() => {
    onDropFile(preview);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [preview]);

  const onDrop = (acceptedFiles: File[]) => {
    const formData = new FormData();
    formData.append('file', acceptedFiles[0]);

    void fetch(uploadUrl, {
      method: 'POST',
      body: formData,
    })
      .then((res) => res.json())
      .then(([{ ipfsHash }]) => {
        const link = `${uploadConfig.gateway}/`;

        if (multi) {
          setPreview((prev) => [...prev, `${link}${ipfsHash}`]);
        } else {
          setPreview([`${link}${ipfsHash}`]);
        }
      });
  };

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    maxFiles: multi ? undefined : 1,
    accept: {
      'image/png': [],
      'image/jpg': [],
      'image/jpeg': [],
    },
    minSize: 760,
  });

  const handleRemovePreview = (e: MouseEvent<HTMLButtonElement>, link?: string) => {
    e.stopPropagation();
    if (multi) {
      setPreview((prev) => prev.filter((item) => item !== link));
    } else {
      setPreview([]);
    }
  };

  return (
    <>
      <div {...getRootProps()} className={cx(styles.dropzone, className || '')}>
        {!preview.length || multi ? (
          <>
            <input {...getInputProps()} />
            {isDragActive ? (
              <div className={cx(styles.label)}>
                <p className={cx(styles['label-title'])}>Drop the files here ...</p>
              </div>
            ) : (
              <>
                {!content ? (
                  <div className={cx(styles.label)}>
                    <img src={picImage} alt="upload" />
                    <h5 className={cx(styles['label-title'])}>Upload photo</h5>
                    <p className={cx(styles['label-description'])}>
                      Image not less than 1280x760 in JPG, JPEG or PNG format, up to 1 MB in size
                    </p>
                  </div>
                ) : (
                  <>{content}</>
                )}
              </>
            )}
          </>
        ) : null}
        {!!preview.length && !multi && (
          <div className={cx(styles['uploaded-pic-wrapper'])}>
            <img src={preview[0]} alt="preview" className={cx(styles['uploaded-pic'])} />
            <Button
              icon={closeIcon}
              variant="text"
              className={cx(styles['close-icon'])}
              label=""
              onClick={handleRemovePreview}
            />
          </div>
        )}
      </div>

      {!!preview.length && multi && (
        <div className={cx(styles['multi-previews-wrapper'])}>
          {preview.map((link) => (
            <div key={link} className={cx(styles['uploaded-pic-wrapper-multi'])}>
              <img src={link} alt="preview" className={cx(styles['uploaded-pic-multi'])} />
              <Button
                icon={closeIcon}
                variant="text"
                className={cx(styles['close-icon-multi'])}
                label=""
                onClick={(e) => handleRemovePreview(e, link)}
              />
            </div>
          ))}
        </div>
      )}
    </>
  );
}

export { DropzoneUploader };
