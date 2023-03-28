#[doc = "Register `SDCMR` reader"]
pub struct R(crate::R<SDCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCMR` writer"]
pub struct W(crate::W<SDCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCMR_SPEC>;
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
impl From<crate::W<SDCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CTB2` reader - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
pub type CTB2_R = crate::BitReader<bool>;
#[doc = "Field `CTB2` writer - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
pub type CTB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDCMR_SPEC, bool, O>;
#[doc = "Field `CTB1` reader - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
pub type CTB1_R = crate::BitReader<bool>;
#[doc = "Field `CTB1` writer - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
pub type CTB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDCMR_SPEC, bool, O>;
#[doc = "Field `NRFS` reader - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
pub type NRFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NRFS` writer - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
pub type NRFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCMR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MRD` reader - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\]
bits are also used to program the extended mode register for mobile SDRAM."]
pub type MRD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MRD` writer - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\]
bits are also used to program the extended mode register for mobile SDRAM."]
pub type MRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCMR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
    #[inline(always)]
    pub fn ctb2(&self) -> CTB2_R {
        CTB2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
    #[inline(always)]
    pub fn ctb1(&self) -> CTB1_R {
        CTB1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:22 - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\]
bits are also used to program the extended mode register for mobile SDRAM."]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
    #[inline(always)]
    #[must_use]
    pub fn ctb2(&mut self) -> CTB2_W<3> {
        CTB2_W::new(self)
    }
    #[doc = "Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
    #[inline(always)]
    #[must_use]
    pub fn ctb1(&mut self) -> CTB1_W<4> {
        CTB1_W::new(self)
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
    #[inline(always)]
    #[must_use]
    pub fn nrfs(&mut self) -> NRFS_W<5> {
        NRFS_W::new(self)
    }
    #[doc = "Bits 9:22 - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\]
bits are also used to program the extended mode register for mobile SDRAM."]
    #[inline(always)]
    #[must_use]
    pub fn mrd(&mut self) -> MRD_W<9> {
        MRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the command issued when the SDRAM device is accessed. This register is used to initialize the SDRAM device, and to activate the Self-refresh and the Power-down modes. As soon as the MODE field is written, the command will be issued only to one or to both SDRAM banks according to CTB1 and CTB2 command bits. This register is the same for both SDRAM banks.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcmr](index.html) module"]
pub struct SDCMR_SPEC;
impl crate::RegisterSpec for SDCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdcmr::R](R) reader structure"]
impl crate::Readable for SDCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdcmr::W](W) writer structure"]
impl crate::Writable for SDCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDCMR to value 0"]
impl crate::Resettable for SDCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
