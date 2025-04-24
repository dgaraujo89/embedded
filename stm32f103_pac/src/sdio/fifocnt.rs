#[doc = "Register `FIFOCNT` reader"]
pub type R = crate::R<FIFOCNT_SPEC>;
#[doc = "Field `FIF0COUNT` reader - FIF0COUNT"]
pub type FIF0COUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - FIF0COUNT"]
    #[inline(always)]
    pub fn fif0count(&self) -> FIF0COUNT_R {
        FIF0COUNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifocnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOCNT_SPEC;
impl crate::RegisterSpec for FIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifocnt::R`](R) reader structure"]
impl crate::Readable for FIFOCNT_SPEC {}
#[doc = "`reset()` method sets FIFOCNT to value 0"]
impl crate::Resettable for FIFOCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
