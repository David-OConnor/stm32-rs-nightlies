#[doc = "Register `ETH_MACPPSTTNR` reader"]
pub struct R(crate::R<ETH_MACPPSTTNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPPSTTNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPPSTTNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPPSTTNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPPSTTNR` writer"]
pub struct W(crate::W<ETH_MACPPSTTNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPPSTTNR_SPEC>;
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
impl From<crate::W<ETH_MACPPSTTNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPPSTTNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTSL0` reader - Target Time Low for PPS Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the value in both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in PPS control register (ETH_MACPPSCR). When the TSCTRLSSR bit is set in the Timestamp control Register (ETH_MACTSCR), this value should not exceed 0x3B9A_C9FF. The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value."]
pub type TTSL0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TTSL0` writer - Target Time Low for PPS Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the value in both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in PPS control register (ETH_MACPPSCR). When the TSCTRLSSR bit is set in the Timestamp control Register (ETH_MACTSCR), this value should not exceed 0x3B9A_C9FF. The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value."]
pub type TTSL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACPPSTTNR_SPEC, u32, u32, 31, O>;
#[doc = "Field `TRGTBUSY0` reader - PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS control register (ETH_MACPPSCR) is programmed to 010 or 011. Programming the PPSCMD0 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers with the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
pub type TRGTBUSY0_R = crate::BitReader<bool>;
#[doc = "Field `TRGTBUSY0` writer - PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS control register (ETH_MACPPSCR) is programmed to 010 or 011. Programming the PPSCMD0 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers with the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
pub type TRGTBUSY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPPSTTNR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the value in both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in PPS control register (ETH_MACPPSCR). When the TSCTRLSSR bit is set in the Timestamp control Register (ETH_MACTSCR), this value should not exceed 0x3B9A_C9FF. The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value."]
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS control register (ETH_MACPPSCR) is programmed to 010 or 011. Programming the PPSCMD0 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers with the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the value in both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in PPS control register (ETH_MACPPSCR). When the TSCTRLSSR bit is set in the Timestamp control Register (ETH_MACTSCR), this value should not exceed 0x3B9A_C9FF. The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value."]
    #[inline(always)]
    #[must_use]
    pub fn ttsl0(&mut self) -> TTSL0_W<0> {
        TTSL0_W::new(self)
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS control register (ETH_MACPPSCR) is programmed to 010 or 011. Programming the PPSCMD0 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers with the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
    #[inline(always)]
    #[must_use]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W<31> {
        TRGTBUSY0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS target time nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppsttnr](index.html) module"]
pub struct ETH_MACPPSTTNR_SPEC;
impl crate::RegisterSpec for ETH_MACPPSTTNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macppsttnr::R](R) reader structure"]
impl crate::Readable for ETH_MACPPSTTNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macppsttnr::W](W) writer structure"]
impl crate::Writable for ETH_MACPPSTTNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACPPSTTNR to value 0"]
impl crate::Resettable for ETH_MACPPSTTNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}