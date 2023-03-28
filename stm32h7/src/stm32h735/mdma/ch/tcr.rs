#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINC` reader - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub type SINC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINC` writer - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
pub type SINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DINC` reader - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
pub type DINC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINC` writer - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
pub type DINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SSIZE` reader - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
pub type SSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSIZE` writer - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
pub type SSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DSIZE` reader - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
pub type DSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSIZE` writer - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SINCOS` reader - source increment offset size"]
pub type SINCOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINCOS` writer - source increment offset size"]
pub type SINCOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DINCOS` reader - Destination increment offset"]
pub type DINCOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINCOS` writer - Destination increment offset"]
pub type DINCOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SBURST` reader - source burst transfer configuration"]
pub type SBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBURST` writer - source burst transfer configuration"]
pub type SBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBURST` reader - Destination burst transfer configuration"]
pub type DBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBURST` writer - Destination burst transfer configuration"]
pub type DBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TLEN` reader - buffer transfer lengh"]
pub type TLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLEN` writer - buffer transfer lengh"]
pub type TLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `PKE` reader - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\]
value. This bit is protected and can be written only if EN is 0"]
pub type PKE_R = crate::BitReader<bool>;
#[doc = "Field `PKE` writer - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\]
value. This bit is protected and can be written only if EN is 0"]
pub type PKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `PAM` reader - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0"]
pub type PAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAM` writer - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0"]
pub type PAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRGM` reader - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
pub type TRGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGM` writer - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
pub type TRGM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SWRM` reader - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
pub type SWRM_R = crate::BitReader<bool>;
#[doc = "Field `SWRM` writer - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
pub type SWRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `BWM` reader - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
pub type BWM_R = crate::BitReader<bool>;
#[doc = "Field `BWM` writer - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
pub type BWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - source increment offset size"]
    #[inline(always)]
    pub fn sincos(&self) -> SINCOS_R {
        SINCOS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Destination increment offset"]
    #[inline(always)]
    pub fn dincos(&self) -> DINCOS_R {
        DINCOS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - source burst transfer configuration"]
    #[inline(always)]
    pub fn sburst(&self) -> SBURST_R {
        SBURST_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Destination burst transfer configuration"]
    #[inline(always)]
    pub fn dburst(&self) -> DBURST_R {
        DBURST_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:24 - buffer transfer lengh"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bit 25 - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\]
value. This bit is protected and can be written only if EN is 0"]
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn trgm(&self) -> TRGM_R {
        TRGM_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn swrm(&self) -> SWRM_R {
        SWRM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
    #[inline(always)]
    pub fn bwm(&self) -> BWM_R {
        BWM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00)."]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SINC_W<0> {
        SINC_W::new(self)
    }
    #[doc = "Bits 2:3 - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DINC_W<2> {
        DINC_W::new(self)
    }
    #[doc = "Bits 4:5 - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
    #[inline(always)]
    #[must_use]
    pub fn ssize(&mut self) -> SSIZE_W<4> {
        SSIZE_W::new(self)
    }
    #[doc = "Bits 6:7 - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<6> {
        DSIZE_W::new(self)
    }
    #[doc = "Bits 8:9 - source increment offset size"]
    #[inline(always)]
    #[must_use]
    pub fn sincos(&mut self) -> SINCOS_W<8> {
        SINCOS_W::new(self)
    }
    #[doc = "Bits 10:11 - Destination increment offset"]
    #[inline(always)]
    #[must_use]
    pub fn dincos(&mut self) -> DINCOS_W<10> {
        DINCOS_W::new(self)
    }
    #[doc = "Bits 12:14 - source burst transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sburst(&mut self) -> SBURST_W<12> {
        SBURST_W::new(self)
    }
    #[doc = "Bits 15:17 - Destination burst transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dburst(&mut self) -> DBURST_W<15> {
        DBURST_W::new(self)
    }
    #[doc = "Bits 18:24 - buffer transfer lengh"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TLEN_W<18> {
        TLEN_W::new(self)
    }
    #[doc = "Bit 25 - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\]
value. This bit is protected and can be written only if EN is 0"]
    #[inline(always)]
    #[must_use]
    pub fn pke(&mut self) -> PKE_W<25> {
        PKE_W::new(self)
    }
    #[doc = "Bits 26:27 - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0"]
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<26> {
        PAM_W::new(self)
    }
    #[doc = "Bits 28:29 - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn trgm(&mut self) -> TRGM_W<28> {
        TRGM_W::new(self)
    }
    #[doc = "Bit 30 - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn swrm(&mut self) -> SWRM_W<30> {
        SWRM_W::new(self)
    }
    #[doc = "Bit 31 - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
    #[inline(always)]
    #[must_use]
    pub fn bwm(&mut self) -> BWM_W<31> {
        BWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
