#[doc = "Register `MACRWUFFR` reader"]
pub type R = crate::R<MACRWUFFR_SPEC>;
#[doc = "Register `MACRWUFFR` writer"]
pub type W = crate::W<MACRWUFFR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACRWUFFR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwuffr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwuffr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACRWUFFR_SPEC;
impl crate::RegisterSpec for MACRWUFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwuffr::R`](R) reader structure"]
impl crate::Readable for MACRWUFFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macrwuffr::W`](W) writer structure"]
impl crate::Writable for MACRWUFFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACRWUFFR to value 0"]
impl crate::Resettable for MACRWUFFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
