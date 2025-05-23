#[doc = "Register `POWER` reader"]
pub type R = crate::R<POWER_SPEC>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<POWER_SPEC>;
#[doc = "Field `PWRCTRL` reader - PWRCTRL"]
pub type PWRCTRL_R = crate::FieldReader;
#[doc = "Field `PWRCTRL` writer - PWRCTRL"]
pub type PWRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<POWER_SPEC> {
        PWRCTRL_W::new(self, 0)
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
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_SPEC;
impl crate::RegisterSpec for POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for POWER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for POWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
