#[doc = "Register `FMC_BWTR3` reader"]
pub struct R(crate::R<FMC_BWTR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BWTR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BWTR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BWTR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_BWTR3` writer"]
pub struct W(crate::W<FMC_BWTR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_BWTR3_SPEC>;
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
impl From<crate::W<FMC_BWTR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_BWTR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDSET` reader - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
pub type ADDSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDSET` writer - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
pub type ADDSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BWTR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDHLD` reader - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
pub type ADDHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDHLD` writer - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
pub type ADDHLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BWTR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATAST` reader - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ..."]
pub type DATAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAST` writer - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ..."]
pub type DATAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BWTR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `BUSTURN` reader - Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ..."]
pub type BUSTURN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSTURN` writer - Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ..."]
pub type BUSTURN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BWTR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACCMOD` reader - Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACCMOD` writer - Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BWTR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATAHLD` reader - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:"]
pub type DATAHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAHLD` writer - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:"]
pub type DATAHLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BWTR3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ..."]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ..."]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:"]
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<0> {
        ADDSET_W::new(self)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<4> {
        ADDHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ..."]
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<8> {
        DATAST_W::new(self)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t<sub>PC</sub> (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ tPC min ..."]
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<16> {
        BUSTURN_W::new(self)
    }
    #[doc = "Bits 28:29 - Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<28> {
        ACCMOD_W::new(self)
    }
    #[doc = "Bits 30:31 - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:"]
    #[inline(always)]
    #[must_use]
    pub fn datahld(&mut self) -> DATAHLD_W<30> {
        DATAHLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR-Flash write timing registers 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bwtr3](index.html) module"]
pub struct FMC_BWTR3_SPEC;
impl crate::RegisterSpec for FMC_BWTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bwtr3::R](R) reader structure"]
impl crate::Readable for FMC_BWTR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_bwtr3::W](W) writer structure"]
impl crate::Writable for FMC_BWTR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_BWTR3 to value 0x0fff_ffff"]
impl crate::Resettable for FMC_BWTR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
