#[doc = "Register `CCMR1_Input` reader"]
pub type R = crate::R<CCMR1_INPUT_SPEC>;
#[doc = "Register `CCMR1_Input` writer"]
pub type W = crate::W<CCMR1_INPUT_SPEC>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler"]
pub type IC1PSC_R = crate::FieldReader;
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler"]
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1F` reader - Input capture 1 filter"]
pub type IC1F_R = crate::FieldReader;
#[doc = "Field `IC1F` writer - Input capture 1 filter"]
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<CCMR1_INPUT_SPEC> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<CCMR1_INPUT_SPEC> {
        IC1PSC_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<CCMR1_INPUT_SPEC> {
        IC1F_W::new(self, 4)
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
#[doc = "capture/compare mode register (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for CCMR1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_input::R`](R) reader structure"]
impl crate::Readable for CCMR1_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_input::W`](W) writer structure"]
impl crate::Writable for CCMR1_INPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR1_Input to value 0"]
impl crate::Resettable for CCMR1_INPUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
