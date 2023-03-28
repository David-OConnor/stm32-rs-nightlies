#[doc = "Register `FMC_SDCR2` reader"]
pub struct R(crate::R<FMC_SDCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_SDCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_SDCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_SDCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_SDCR2` writer"]
pub struct W(crate::W<FMC_SDCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_SDCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FMC_SDCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_SDCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NC` reader - Number of column address bits These bits define the number of bits of a column address."]
pub type NC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NC` writer - Number of column address bits These bits define the number of bits of a column address."]
pub type NC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_SDCR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `NR` reader - Number of row address bits These bits define the number of bits of a row address."]
pub type NR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NR` writer - Number of row address bits These bits define the number of bits of a row address."]
pub type NR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_SDCR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `MWID` reader - Memory data bus width. These bits define the memory device width."]
pub type MWID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MWID` writer - Memory data bus width. These bits define the memory device width."]
pub type MWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_SDCR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `NB` reader - Number of internal banks This bit sets the number of internal banks."]
pub type NB_R = crate::BitReader<bool>;
#[doc = "Field `NB` writer - Number of internal banks This bit sets the number of internal banks."]
pub type NB_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SDCR2_SPEC, bool, O>;
#[doc = "Field `CAS` reader - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles"]
pub type CAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAS` writer - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles"]
pub type CAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_SDCR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `WP` reader - Write protection This bit enables write mode access to the SDRAM bank."]
pub type WP_R = crate::BitReader<bool>;
#[doc = "Field `WP` writer - Write protection This bit enables write mode access to the SDRAM bank."]
pub type WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SDCR2_SPEC, bool, O>;
#[doc = "Field `SDCLK` reader - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care."]
pub type SDCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDCLK` writer - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care."]
pub type SDCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_SDCR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `RBURST` reader - Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care."]
pub type RBURST_R = crate::BitReader<bool>;
#[doc = "Field `RBURST` writer - Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care."]
pub type RBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_SDCR2_SPEC, bool, O>;
#[doc = "Field `RPIPE` reader - Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
pub type RPIPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPIPE` writer - Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
pub type RPIPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_SDCR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Number of column address bits These bits define the number of bits of a column address."]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of row address bits These bits define the number of bits of a row address."]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Memory data bus width. These bits define the memory device width."]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Number of internal banks This bit sets the number of internal banks."]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Write protection This bit enables write mode access to the SDRAM bank."]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care."]
    #[inline(always)]
    pub fn sdclk(&self) -> SDCLK_R {
        SDCLK_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care."]
    #[inline(always)]
    pub fn rburst(&self) -> RBURST_R {
        RBURST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
    #[inline(always)]
    pub fn rpipe(&self) -> RPIPE_R {
        RPIPE_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of column address bits These bits define the number of bits of a column address."]
    #[inline(always)]
    #[must_use]
    pub fn nc(&mut self) -> NC_W<0> {
        NC_W::new(self)
    }
    #[doc = "Bits 2:3 - Number of row address bits These bits define the number of bits of a row address."]
    #[inline(always)]
    #[must_use]
    pub fn nr(&mut self) -> NR_W<2> {
        NR_W::new(self)
    }
    #[doc = "Bits 4:5 - Memory data bus width. These bits define the memory device width."]
    #[inline(always)]
    #[must_use]
    pub fn mwid(&mut self) -> MWID_W<4> {
        MWID_W::new(self)
    }
    #[doc = "Bit 6 - Number of internal banks This bit sets the number of internal banks."]
    #[inline(always)]
    #[must_use]
    pub fn nb(&mut self) -> NB_W<6> {
        NB_W::new(self)
    }
    #[doc = "Bits 7:8 - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CAS_W<7> {
        CAS_W::new(self)
    }
    #[doc = "Bit 9 - Write protection This bit enables write mode access to the SDRAM bank."]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<9> {
        WP_W::new(self)
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care."]
    #[inline(always)]
    #[must_use]
    pub fn sdclk(&mut self) -> SDCLK_W<10> {
        SDCLK_W::new(self)
    }
    #[doc = "Bit 12 - Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care."]
    #[inline(always)]
    #[must_use]
    pub fn rburst(&mut self) -> RBURST_W<12> {
        RBURST_W::new(self)
    }
    #[doc = "Bits 13:14 - Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
    #[inline(always)]
    #[must_use]
    pub fn rpipe(&mut self) -> RPIPE_W<13> {
        RPIPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM control registers 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sdcr2](index.html) module"]
pub struct FMC_SDCR2_SPEC;
impl crate::RegisterSpec for FMC_SDCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_sdcr2::R](R) reader structure"]
impl crate::Readable for FMC_SDCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_sdcr2::W](W) writer structure"]
impl crate::Writable for FMC_SDCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_SDCR2 to value 0x02d0"]
impl crate::Resettable for FMC_SDCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x02d0;
}
