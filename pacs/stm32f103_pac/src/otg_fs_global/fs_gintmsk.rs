#[doc = "Register `FS_GINTMSK` reader"]
pub type R = crate::R<FS_GINTMSK_SPEC>;
#[doc = "Register `FS_GINTMSK` writer"]
pub type W = crate::W<FS_GINTMSK_SPEC>;
#[doc = "Field `MMISM` reader - Mode mismatch interrupt mask"]
pub type MMISM_R = crate::BitReader;
#[doc = "Field `MMISM` writer - Mode mismatch interrupt mask"]
pub type MMISM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - OTG interrupt mask"]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `OTGINT` writer - OTG interrupt mask"]
pub type OTGINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFM` reader - Start of frame mask"]
pub type SOFM_R = crate::BitReader;
#[doc = "Field `SOFM` writer - Start of frame mask"]
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVLM` reader - Receive FIFO non-empty mask"]
pub type RXFLVLM_R = crate::BitReader;
#[doc = "Field `RXFLVLM` writer - Receive FIFO non-empty mask"]
pub type RXFLVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEM` reader - Non-periodic TxFIFO empty mask"]
pub type NPTXFEM_R = crate::BitReader;
#[doc = "Field `NPTXFEM` writer - Non-periodic TxFIFO empty mask"]
pub type NPTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINAKEFFM` reader - Global non-periodic IN NAK effective mask"]
pub type GINAKEFFM_R = crate::BitReader;
#[doc = "Field `GINAKEFFM` writer - Global non-periodic IN NAK effective mask"]
pub type GINAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GONAKEFFM` reader - Global OUT NAK effective mask"]
pub type GONAKEFFM_R = crate::BitReader;
#[doc = "Field `GONAKEFFM` writer - Global OUT NAK effective mask"]
pub type GONAKEFFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESUSPM` reader - Early suspend mask"]
pub type ESUSPM_R = crate::BitReader;
#[doc = "Field `ESUSPM` writer - Early suspend mask"]
pub type ESUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPM` reader - USB suspend mask"]
pub type USBSUSPM_R = crate::BitReader;
#[doc = "Field `USBSUSPM` writer - USB suspend mask"]
pub type USBSUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB reset mask"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset mask"]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDNEM` reader - Enumeration done mask"]
pub type ENUMDNEM_R = crate::BitReader;
#[doc = "Field `ENUMDNEM` writer - Enumeration done mask"]
pub type ENUMDNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOODRPM` reader - Isochronous OUT packet dropped interrupt mask"]
pub type ISOODRPM_R = crate::BitReader;
#[doc = "Field `ISOODRPM` writer - Isochronous OUT packet dropped interrupt mask"]
pub type ISOODRPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFM` reader - End of periodic frame interrupt mask"]
pub type EOPFM_R = crate::BitReader;
#[doc = "Field `EOPFM` writer - End of periodic frame interrupt mask"]
pub type EOPFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPMISM` reader - Endpoint mismatch interrupt mask"]
pub type EPMISM_R = crate::BitReader;
#[doc = "Field `EPMISM` writer - Endpoint mismatch interrupt mask"]
pub type EPMISM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - IN endpoints interrupt mask"]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `IEPINT` writer - IN endpoints interrupt mask"]
pub type IEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPINT` reader - OUT endpoints interrupt mask"]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `OEPINT` writer - OUT endpoints interrupt mask"]
pub type OEPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IISOIXFRM` reader - Incomplete isochronous IN transfer mask"]
pub type IISOIXFRM_R = crate::BitReader;
#[doc = "Field `IISOIXFRM` writer - Incomplete isochronous IN transfer mask"]
pub type IISOIXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPXFRM_IISOOXFRM` reader - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub type IPXFRM_IISOOXFRM_R = crate::BitReader;
#[doc = "Field `IPXFRM_IISOOXFRM` writer - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub type IPXFRM_IISOOXFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTIM` reader - Host port interrupt mask"]
pub type PRTIM_R = crate::BitReader;
#[doc = "Field `HCIM` reader - Host channels interrupt mask"]
pub type HCIM_R = crate::BitReader;
#[doc = "Field `HCIM` writer - Host channels interrupt mask"]
pub type HCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEM` reader - Periodic TxFIFO empty mask"]
pub type PTXFEM_R = crate::BitReader;
#[doc = "Field `PTXFEM` writer - Periodic TxFIFO empty mask"]
pub type PTXFEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIDSCHGM` reader - Connector ID status change mask"]
pub type CIDSCHGM_R = crate::BitReader;
#[doc = "Field `CIDSCHGM` writer - Connector ID status change mask"]
pub type CIDSCHGM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCINT` reader - Disconnect detected interrupt mask"]
pub type DISCINT_R = crate::BitReader;
#[doc = "Field `DISCINT` writer - Disconnect detected interrupt mask"]
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRQIM` reader - Session request/new session detected interrupt mask"]
pub type SRQIM_R = crate::BitReader;
#[doc = "Field `SRQIM` writer - Session request/new session detected interrupt mask"]
pub type SRQIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUIM` reader - Resume/remote wakeup detected interrupt mask"]
pub type WUIM_R = crate::BitReader;
#[doc = "Field `WUIM` writer - Resume/remote wakeup detected interrupt mask"]
pub type WUIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint mismatch interrupt mask"]
    #[inline(always)]
    pub fn epmism(&self) -> EPMISM_R {
        EPMISM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    pub fn ipxfrm_iisooxfrm(&self) -> IPXFRM_IISOOXFRM_R {
        IPXFRM_IISOOXFRM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn mmism(&mut self) -> MMISM_W<FS_GINTMSK_SPEC> {
        MMISM_W::new(self, 1)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn otgint(&mut self) -> OTGINT_W<FS_GINTMSK_SPEC> {
        OTGINT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<FS_GINTMSK_SPEC> {
        SOFM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W<FS_GINTMSK_SPEC> {
        RXFLVLM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfem(&mut self) -> NPTXFEM_W<FS_GINTMSK_SPEC> {
        NPTXFEM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W<FS_GINTMSK_SPEC> {
        GINAKEFFM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W<FS_GINTMSK_SPEC> {
        GONAKEFFM_W::new(self, 7)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    #[must_use]
    pub fn esuspm(&mut self) -> ESUSPM_W<FS_GINTMSK_SPEC> {
        ESUSPM_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W<FS_GINTMSK_SPEC> {
        USBSUSPM_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<FS_GINTMSK_SPEC> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    #[must_use]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W<FS_GINTMSK_SPEC> {
        ENUMDNEM_W::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W<FS_GINTMSK_SPEC> {
        ISOODRPM_W::new(self, 14)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn eopfm(&mut self) -> EOPFM_W<FS_GINTMSK_SPEC> {
        EOPFM_W::new(self, 15)
    }
    #[doc = "Bit 17 - Endpoint mismatch interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epmism(&mut self) -> EPMISM_W<FS_GINTMSK_SPEC> {
        EPMISM_W::new(self, 17)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn iepint(&mut self) -> IEPINT_W<FS_GINTMSK_SPEC> {
        IEPINT_W::new(self, 18)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn oepint(&mut self) -> OEPINT_W<FS_GINTMSK_SPEC> {
        OEPINT_W::new(self, 19)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    #[must_use]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W<FS_GINTMSK_SPEC> {
        IISOIXFRM_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ipxfrm_iisooxfrm(&mut self) -> IPXFRM_IISOOXFRM_W<FS_GINTMSK_SPEC> {
        IPXFRM_IISOOXFRM_W::new(self, 21)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn hcim(&mut self) -> HCIM_W<FS_GINTMSK_SPEC> {
        HCIM_W::new(self, 25)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfem(&mut self) -> PTXFEM_W<FS_GINTMSK_SPEC> {
        PTXFEM_W::new(self, 26)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    #[must_use]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W<FS_GINTMSK_SPEC> {
        CIDSCHGM_W::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn discint(&mut self) -> DISCINT_W<FS_GINTMSK_SPEC> {
        DISCINT_W::new(self, 29)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn srqim(&mut self) -> SRQIM_W<FS_GINTMSK_SPEC> {
        SRQIM_W::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wuim(&mut self) -> WUIM_W<FS_GINTMSK_SPEC> {
        WUIM_W::new(self, 31)
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
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_GINTMSK_SPEC;
impl crate::RegisterSpec for FS_GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_gintmsk::R`](R) reader structure"]
impl crate::Readable for FS_GINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fs_gintmsk::W`](W) writer structure"]
impl crate::Writable for FS_GINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_GINTMSK to value 0"]
impl crate::Resettable for FS_GINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
