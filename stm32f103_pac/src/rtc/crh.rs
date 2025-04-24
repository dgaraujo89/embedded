#[doc = "Register `CRH` reader"]
pub type R = crate::R<CRH_SPEC>;
#[doc = "Register `CRH` writer"]
pub type W = crate::W<CRH_SPEC>;
#[doc = "Field `SECIE` reader - Second interrupt Enable"]
pub type SECIE_R = crate::BitReader;
#[doc = "Field `SECIE` writer - Second interrupt Enable"]
pub type SECIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRIE` reader - Alarm interrupt Enable"]
pub type ALRIE_R = crate::BitReader;
#[doc = "Field `ALRIE` writer - Alarm interrupt Enable"]
pub type ALRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWIE` reader - Overflow interrupt Enable"]
pub type OWIE_R = crate::BitReader;
#[doc = "Field `OWIE` writer - Overflow interrupt Enable"]
pub type OWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn secie(&mut self) -> SECIE_W<CRH_SPEC> {
        SECIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrie(&mut self) -> ALRIE_W<CRH_SPEC> {
        ALRIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn owie(&mut self) -> OWIE_W<CRH_SPEC> {
        OWIE_W::new(self, 2)
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
#[doc = "RTC Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRH_SPEC;
impl crate::RegisterSpec for CRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crh::R`](R) reader structure"]
impl crate::Readable for CRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crh::W`](W) writer structure"]
impl crate::Writable for CRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRH to value 0"]
impl crate::Resettable for CRH_SPEC {
    const RESET_VALUE: u32 = 0;
}
