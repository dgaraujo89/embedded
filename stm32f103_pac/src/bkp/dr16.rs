#[doc = "Register `DR16` reader"]
pub type R = crate::R<DR16_SPEC>;
#[doc = "Register `DR16` writer"]
pub type W = crate::W<DR16_SPEC>;
#[doc = "Field `D16` reader - Backup data"]
pub type D16_R = crate::FieldReader<u16>;
#[doc = "Field `D16` writer - Backup data"]
pub type D16_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d16(&self) -> D16_R {
        D16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d16(&mut self) -> D16_W<DR16_SPEC> {
        D16_W::new(self, 0)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR16_SPEC;
impl crate::RegisterSpec for DR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr16::R`](R) reader structure"]
impl crate::Readable for DR16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr16::W`](W) writer structure"]
impl crate::Writable for DR16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR16 to value 0"]
impl crate::Resettable for DR16_SPEC {
    const RESET_VALUE: u32 = 0;
}
