export default function DeleteAccount() {
    return (
        <div className="absolute top-1/2 left-1/2 w-[770px] h-[318px] rounded-[20px] p-6">
            <div className="flex items-center justify-end">
                <span className="mb-4"> Your account is about to be deleted </span>
                <span className="mb-10">
                    All data will be lost and you wont be able to recover it. <br />
                    Are you sure you want to delete your account?
                </span>

                {/* Delete & Go back button */}
            </div>
        </div>
    )
}