#[doc = "Register `MACA2HR` reader"]
pub type R = crate::R<MACA2HR_SPEC>;
#[doc = "Register `MACA2HR` writer"]
pub type W = crate::W<MACA2HR_SPEC>;
#[doc = "Field `ETH_MACA2HR` reader - Ethernet MAC address 2 high register"]
pub type ETH_MACA2HR_R = crate::FieldReader<u16>;
#[doc = "Field `ETH_MACA2HR` writer - Ethernet MAC address 2 high register"]
pub type ETH_MACA2HR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask byte control"]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask byte control"]
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source address"]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - Source address"]
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address enable"]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - Address enable"]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Ethernet MAC address 2 high register"]
    #[inline(always)]
    pub fn eth_maca2hr(&self) -> ETH_MACA2HR_R {
        ETH_MACA2HR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ethernet MAC address 2 high register"]
    #[inline(always)]
    #[must_use]
    pub fn eth_maca2hr(&mut self) -> ETH_MACA2HR_W<MACA2HR_SPEC> {
        ETH_MACA2HR_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<MACA2HR_SPEC> {
        MBC_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<MACA2HR_SPEC> {
        SA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<MACA2HR_SPEC> {
        AE_W::new(self, 31)
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
#[doc = "Ethernet MAC address 2 high register (ETH_MACA2HR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA2HR_SPEC;
impl crate::RegisterSpec for MACA2HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2hr::R`](R) reader structure"]
impl crate::Readable for MACA2HR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca2hr::W`](W) writer structure"]
impl crate::Writable for MACA2HR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA2HR to value 0x50"]
impl crate::Resettable for MACA2HR_SPEC {
    const RESET_VALUE: u32 = 0x50;
}
