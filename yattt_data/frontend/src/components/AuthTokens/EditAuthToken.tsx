import {
  Button,
  FormControl,
  FormErrorMessage,
  FormLabel,
  Input,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
} from "@chakra-ui/react"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { type SubmitHandler, useForm } from "react-hook-form"

import {
  type ApiError,
  type AuthTokenPublic,
  type AuthTokenUpdate,
  AuthTokensService,
} from "../../client"
import useCustomToast from "../../hooks/useCustomToast"
import { handleError } from "../../utils"

interface EditItemProps {
  item: AuthTokenPublic
  isOpen: boolean
  onClose: () => void
}

const EditAuthToken = ({ item, isOpen, onClose }: EditItemProps) => {
  const queryClient = useQueryClient()
  const showToast = useCustomToast()
  const {
    register,
    handleSubmit,
    reset,
    formState: { isSubmitting, errors, isDirty },
  } = useForm<AuthTokenUpdate>({
    mode: "onBlur",
    criteriaMode: "all",
    defaultValues: item,
  })

  const mutation = useMutation({
    mutationFn: (data: AuthTokenUpdate) =>
      AuthTokensService.updateAuthToken({ id: item.id, requestBody: data }),
    onSuccess: () => {
      showToast("Success!", "AuthToken updated successfully.", "success")
      onClose()
    },
    onError: (err: ApiError) => {
      handleError(err, showToast)
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["auth-tokens"] })
    },
  })

  const onSubmit: SubmitHandler<AuthTokenUpdate> = async (data) => {
    mutation.mutate(data)
  }

  const onCancel = () => {
    reset()
    onClose()
  }

  return (
    <>
      <Modal
        isOpen={isOpen}
        onClose={onClose}
        size={{ base: "sm", md: "md" }}
        isCentered
      >
        <ModalOverlay />
        <ModalContent as="form" onSubmit={handleSubmit(onSubmit)}>
          <ModalHeader>Edit AuthToken</ModalHeader>
          <ModalCloseButton />
          <ModalBody pb={6}>
            <FormControl isInvalid={!!errors.tag_id}>
              <FormLabel htmlFor="tag_id">TagId</FormLabel>
              <Input
                  id="tag_id"
                  {...register('tag_id', {
                    required: 'tag_id is required',
                  })}
                  type="text"
              />
              {errors.tag_id && (
                  <FormErrorMessage>{errors.tag_id.message}</FormErrorMessage>
              )}
            </FormControl>
            <FormControl isInvalid={!!errors.device_id}>
              <FormLabel htmlFor="device_id">DeviceId</FormLabel>
              <Input
                  id="device_id"
                  {...register('device_id', {
                    required: 'device_id is required',
                  })}
                  type="text"
              />
              {errors.device_id && (
                  <FormErrorMessage>{errors.device_id.message}</FormErrorMessage>
              )}
            </FormControl>
            <FormControl mt={4}>
              <FormLabel htmlFor="description">Description</FormLabel>
              <Input
                  id="description"
                  {...register('description')}
                  placeholder="Description"
                  type="text"
              />
            </FormControl>
          </ModalBody>
          <ModalFooter gap={3}>
            <Button
              variant="primary"
              type="submit"
              isLoading={isSubmitting}
              isDisabled={!isDirty}
            >
              Save
            </Button>
            <Button onClick={onCancel}>Cancel</Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  )
}

export default EditAuthToken
