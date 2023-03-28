#[doc = "Register `C7TR1` reader"]
pub struct R(crate::R<C7TR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C7TR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C7TR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C7TR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C7TR1` writer"]
pub struct W(crate::W<C7TR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C7TR1_SPEC>;
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
impl From<crate::W<C7TR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C7TR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDW_LOG2` reader - binary logarithm of the source data width of a burst in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
pub type SDW_LOG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDW_LOG2` writer - binary logarithm of the source data width of a burst in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
pub type SDW_LOG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7TR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SINC` reader - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type SINC_R = crate::BitReader<bool>;
#[doc = "Field `SINC` writer - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type SINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7TR1_SPEC, bool, O>;
#[doc = "Field `SBL_1` reader - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type SBL_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBL_1` writer - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type SBL_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7TR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `PAM` reader - padding/alignment mode If DDW_LOG2\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported."]
pub type PAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAM` writer - padding/alignment mode If DDW_LOG2\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported."]
pub type PAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7TR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SBX` reader - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
pub type SBX_R = crate::BitReader<bool>;
#[doc = "Field `SBX` writer - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
pub type SBX_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7TR1_SPEC, bool, O>;
#[doc = "Field `SAP` reader - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type SAP_R = crate::BitReader<bool>;
#[doc = "Field `SAP` writer - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type SAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7TR1_SPEC, bool, O>;
#[doc = "Field `SSEC` reader - security attribute of the GPDMA transfer from the source If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx =1 . A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure."]
pub type SSEC_R = crate::BitReader<bool>;
#[doc = "Field `SSEC` writer - security attribute of the GPDMA transfer from the source If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx =1 . A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure."]
pub type SSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7TR1_SPEC, bool, O>;
#[doc = "Field `DDW_LOG2` reader - binary logarithm of the destination data width of a burst, in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\]
and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
pub type DDW_LOG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDW_LOG2` writer - binary logarithm of the destination data width of a burst, in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\]
and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
pub type DDW_LOG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7TR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DINC` reader - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type DINC_R = crate::BitReader<bool>;
#[doc = "Field `DINC` writer - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type DINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7TR1_SPEC, bool, O>;
#[doc = "Field `DBL_1` reader - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type DBL_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBL_1` writer - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type DBL_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7TR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `DBX` reader - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
pub type DBX_R = crate::BitReader<bool>;
#[doc = "Field `DBX` writer - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
pub type DBX_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7TR1_SPEC, bool, O>;
#[doc = "Field `DHX` reader - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
pub type DHX_R = crate::BitReader<bool>;
#[doc = "Field `DHX` writer - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
pub type DHX_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7TR1_SPEC, bool, O>;
#[doc = "Field `DAP` reader - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type DAP_R = crate::BitReader<bool>;
#[doc = "Field `DAP` writer - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type DAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7TR1_SPEC, bool, O>;
#[doc = "Field `DSEC` reader - security attribute of the GPDMA transfer to the destination If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure."]
pub type DSEC_R = crate::BitReader<bool>;
#[doc = "Field `DSEC` writer - security attribute of the GPDMA transfer to the destination If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure."]
pub type DSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7TR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - binary logarithm of the source data width of a burst in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    pub fn sdw_log2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    pub fn sbl_1(&self) -> SBL_1_R {
        SBL_1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 11:12 - padding/alignment mode If DDW_LOG2\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported."]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
    #[inline(always)]
    pub fn sbx(&self) -> SBX_R {
        SBX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn sap(&self) -> SAP_R {
        SAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - security attribute of the GPDMA transfer from the source If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx =1 . A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure."]
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\]
and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn ddw_log2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    pub fn dbl_1(&self) -> DBL_1_R {
        DBL_1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
    #[inline(always)]
    pub fn dbx(&self) -> DBX_R {
        DBX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
    #[inline(always)]
    pub fn dhx(&self) -> DHX_R {
        DHX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn dap(&self) -> DAP_R {
        DAP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - security attribute of the GPDMA transfer to the destination If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure."]
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - binary logarithm of the source data width of a burst in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn sdw_log2(&mut self) -> SDW_LOG2_W<0> {
        SDW_LOG2_W::new(self)
    }
    #[doc = "Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SINC_W<3> {
        SINC_W::new(self)
    }
    #[doc = "Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn sbl_1(&mut self) -> SBL_1_W<4> {
        SBL_1_W::new(self)
    }
    #[doc = "Bits 11:12 - padding/alignment mode If DDW_LOG2\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<11> {
        PAM_W::new(self)
    }
    #[doc = "Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
    #[inline(always)]
    #[must_use]
    pub fn sbx(&mut self) -> SBX_W<13> {
        SBX_W::new(self)
    }
    #[doc = "Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn sap(&mut self) -> SAP_W<14> {
        SAP_W::new(self)
    }
    #[doc = "Bit 15 - security attribute of the GPDMA transfer from the source If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx =1 . A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn ssec(&mut self) -> SSEC_W<15> {
        SSEC_W::new(self)
    }
    #[doc = "Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\]
and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn ddw_log2(&mut self) -> DDW_LOG2_W<16> {
        DDW_LOG2_W::new(self)
    }
    #[doc = "Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DINC_W<19> {
        DINC_W::new(self)
    }
    #[doc = "Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn dbl_1(&mut self) -> DBL_1_W<20> {
        DBL_1_W::new(self)
    }
    #[doc = "Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
    #[inline(always)]
    #[must_use]
    pub fn dbx(&mut self) -> DBX_W<26> {
        DBX_W::new(self)
    }
    #[doc = "Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
    #[inline(always)]
    #[must_use]
    pub fn dhx(&mut self) -> DHX_W<27> {
        DHX_W::new(self)
    }
    #[doc = "Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn dap(&mut self) -> DAP_W<30> {
        DAP_W::new(self)
    }
    #[doc = "Bit 31 - security attribute of the GPDMA transfer to the destination If GPDMA_SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored when GPDMA_SECCFGR.SECx = 0. When GPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn dsec(&mut self) -> DSEC_W<31> {
        DSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA channel 7 transfer register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7tr1](index.html) module"]
pub struct C7TR1_SPEC;
impl crate::RegisterSpec for C7TR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c7tr1::R](R) reader structure"]
impl crate::Readable for C7TR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c7tr1::W](W) writer structure"]
impl crate::Writable for C7TR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C7TR1 to value 0"]
impl crate::Resettable for C7TR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
