#[doc = "Register `CPAR7` reader"]
pub type R = crate::R<CPAR7_SPEC>;
#[doc = "Register `CPAR7` writer"]
pub type W = crate::W<CPAR7_SPEC>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<CPAR7_SPEC> {
        PA_W::new(self, 0)
    }
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
#[doc = "DMA channel 7 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPAR7_SPEC;
impl crate::RegisterSpec for CPAR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpar7::R`](R) reader structure"]
impl crate::Readable for CPAR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpar7::W`](W) writer structure"]
impl crate::Writable for CPAR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPAR7 to value 0"]
impl crate::Resettable for CPAR7_SPEC {
    const RESET_VALUE: u32 = 0;
}
